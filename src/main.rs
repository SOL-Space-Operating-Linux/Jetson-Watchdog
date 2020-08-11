
extern crate difference;
#[macro_use]
extern crate crossbeam_channel;
extern crate regex;
extern crate serde_json;
extern crate serde;
extern crate serde_derive;
mod watchdog_daemon;
mod log_daemon; // Our implementation of a dmesg watcher
use serde_json::{json,Value, Error};
use serde::{Serialize, Deserialize};

use regex::Regex;
use crossbeam_channel::unbounded;
use chrono::{Datelike, Timelike, Utc};
use std::{thread, time, fs, fs::File};
use std::io::{prelude::*, Seek, Read, Write, SeekFrom};
use std::process::{Command, Stdio};

use nix::sys::wait::waitpid;
use nix::unistd::{fork, getpid, getppid, ForkResult};

// USAGE: sudo ~/jetson-watchdog

//global check register

/*------------------ HELPER FUNCTIONS --------------------*/
/* Function that returns the first element of a vector without destroying it */
fn first<T>(v: &Vec<T>) -> Option<&T> {
    if !v.is_empty() {
        serde::export::Some(v.first().unwrap())
    }
    else {
        return None
    }
}
/* Function that returns the last element of a vector without destroying it */
fn last<T>(v: &Vec<T>) -> Option<&T> {
    if !v.is_empty() {
        serde::export::Some(v.last().unwrap())
    }
    else {
        return None
    }

}
/*Function to be called on a thread that sleeps for one second and then sends a message to the receiver */
fn taskmaster_thread(sender: crossbeam_channel::Sender<String>) {
    let message = "Timer interrupt";
    let one_second = time::Duration::from_secs(5);
    loop {
        thread::sleep(one_second);
        sender.try_send(message.to_string()).unwrap();
    }
}

/*---------------MAIN--------------------*/

fn main() {
//TODO: Put this in its own function
    // Generate, format and assemble the current date string in UTC
    let now = Utc::now();
    let (is_common_era, year) = now.year_ce();
    let mut todays_date = format!("./error-logs/{}-{:0>2}-{:0>2}_{:0>2}:{:0>2}:{:0>2}.json", 
                        year.to_string(),
                        now.month().to_string(),
                        now.day().to_string(),
                        now.hour().to_string(),
                        now.minute().to_string(),
                        now.second().to_string());
                // Open file
    let mut file = fs::OpenOptions::new()
                        .read(true)                        
                        .write(true)                        
                        .create(true)
                        .open(todays_date.to_string()) // ./error-logs/year-month-date hour-minute-second.json
                        .expect("Unable to open file");
    // append to the file           
    write!(&file,"{{}}").expect("unable to write out to file");

    let mut last_error_timestamp: f64 = 0.0;

    // Discard oldest log after thirty days

    let rotate_logs =   Command::new("find")
                                .arg("./error-logs")
                                .arg("-mtime")
                                .arg("+30")
                                .arg("-delete")
                                .spawn()
                                .expect("Failed to rotate logs");

    //load config, use flight defaults if no file, or parts missing

    //wait for boot to finish

    //get baseline ps and lshw
    println!("Starting Daemon");
    
 //   watchdog_daemon::start_watchdog_daemon();

    println!("Daemon Started!!!");
    //check # resets to see aliveness, compare against previous baselines, log diffs
    //update new baseline, # resets

/* -----------------------------------------LOG DAEMON WORK AREA----------------------------------------------*/
    // Create the regex for local parse and sort use 
    let re = Regex::new(r"(\[.?[0-9]+\.[0-9]+\])(.*?)(SBE ERR|SError detected|CPU Memory Error|Machine Check Error|GPU L2|generated a mmu fault|SDHCI_INT_DATA_TIMEOUT|Timeout waiting for hardware interrupt|watchdog detected)").unwrap();
    // Create the vectors to hold timestamps
    let mut all_errors_vec: Vec<f64> = Vec::new();
    let mut sbe_err_vec: Vec<f64> = Vec::new();
    let mut serror_vec: Vec<f64> = Vec::new();
    let mut cpu_mem_vec: Vec<f64> = Vec::new(); 
    let mut cce_machine_vec: Vec<f64> = Vec::new(); 
    let mut gpu_l2_vec: Vec<f64> = Vec::new();
    let mut mmu_fault_vec: Vec<f64> = Vec::new(); 
    let mut flash_write_vec: Vec<f64> = Vec::new(); 
    let mut flash_read_vec: Vec<f64> = Vec::new(); 
    let mut watchdog_detected_vec: Vec<f64> = Vec::new(); 
    //Set up channels, send the receiver over to log_daemon to communicate back
    let (s, receiver) = unbounded();

/* This portion of the program creates the dmesg -w thread and then spawns a process, log_daemon, which watches that thread*/
// TODO: This needs to return some value to show that log_daemon, or dmesg, died.     
    let sender = s.clone(); // clone the sender to send off to the thread
    let dmesg_sender = s.clone();
    
        // TODO: capture OS failure to create the thread 
    let mut dmesg_child = match Command::new("dmesg")
                                    .arg("-w")
                                    .stdout(Stdio::piped())
                                    .spawn()
        {
            Ok(child) => child,
            Err(_) => {
                println!("Failed to create the dmesg child");
                return; // head home early 
            }
        };
    // Set up thread builder; this will let us capture an io::Result if the OS fails to create it
    let dmesg_builder = thread::Builder::new();
    let logdaemon_handle = dmesg_builder.spawn(move || {
        log_daemon::startup(sender, &mut dmesg_child); // a separate thread launches this. 
    }); // transfers ownership of the thread to log_daemon process

/*-------------End log_daemon section-----------*/

// Create a thread which serves as the heartbeat, sleeping and then periodically sending a message
    let interrupt_builder = thread::Builder::new();
    let interrupt_sender = s.clone(); // clone the sender to send off to the thread
    let taskmaster_timer = interrupt_builder.spawn( || {
        taskmaster_thread(interrupt_sender);
    }).unwrap();
    // TODO: This needs to return some value to show that it, or dmesg, died. 

    let sys_time = time::SystemTime::now();
    loop {  //-------------BEGIN MAIN LOOP-----------------
        let our_string = receiver.recv().unwrap().to_string(); // this blocks until there is a message on the channel.

        if our_string.eq("Timer interrupt") {
            println!("Timer interrupt detected!");

            if all_errors_vec.len() > 0 {
                println!("Making new json");
            // create a json object, append to file                 
                let mut new_json = json!({
                        "all_errors_duration": [first(&all_errors_vec), last(&all_errors_vec)],
                        "sbe_err_vec": sbe_err_vec.len(),
                        "serror_vec" : serror_vec.len(),
                        "cpu_mem_vec": cpu_mem_vec.len(),
                        "cce_machine_vec": cce_machine_vec.len(),
                        "gpu_l2_vec": gpu_l2_vec.len(),
                        "mmu_fault_vec": mmu_fault_vec.len(),
                        "flash_write_vec": flash_write_vec.len(),
                        "flash_read_vec": flash_read_vec.len(),
                        "watchdog_detected_vec":watchdog_detected_vec.len() 
                });

                // Open file
                let mut file = fs::OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)                                    
                                    .open(todays_date.to_string()) // ./error-logs/year-month-date hour-minute-second.json
                                    .expect("Unable to open file");
                file.seek(std::io::SeekFrom::End(-1)).unwrap();
                let mut json = serde_json::to_string(&new_json).unwrap();
                json.push('}');
                let file_metadata = file.metadata().unwrap();
                // if filesize is larger than 2, first insert a comma. Otherwise, this is the first object in the file. 
                if file_metadata.len() > 2 {                    
                    println!("the file already has a json in it!");
                    write!(&file,",").expect("Unable to write to file");
                    // file.seek(std::io::SeekFrom::End(-1)).unwrap();
                }
                write!(&file, "\"{:?}\":", first(&all_errors_vec).unwrap()).expect("unable to write out to file");
                // append to the file           
                file.write_all(json.as_bytes()).expect("unable to write out to file");
                println!("Time elapsed: {:?}", sys_time.elapsed().unwrap());
    
                // clear the vectors
                all_errors_vec.clear(); 
                sbe_err_vec.clear();
                serror_vec.clear();
                cpu_mem_vec.clear();
                cce_machine_vec.clear();
                gpu_l2_vec.clear();
                mmu_fault_vec.clear();
                flash_write_vec.clear();
                flash_read_vec.clear();
                watchdog_detected_vec.clear();
            }
        }
        // This section processes incoming commands. 
        else if !our_string.eq("Lost dmesg process") && !our_string.eq("Timer interrupt"){ 
            for cap in re.captures_iter(&our_string) {
                let error_type = cap.get(3).unwrap().as_str(); // take the third argument of the regex, which is the error message
                let raw_timestamp = cap.get(1).unwrap().as_str().replace("[", "").replace("]", "").replace(" ", ""); // take the timestamp
                let timestamp = raw_timestamp.parse::<f64>().unwrap(); // FIXME: can we process this as a string?
                // Check if this is a repeat dmesg feed from the thread being respawned
                if timestamp > last_error_timestamp {
                    last_error_timestamp = timestamp;
                    // save the timestamp on the global errors vector, then according the individual error type
                    all_errors_vec.push(timestamp);
                    match error_type { // switch-case statement for processing each error
                        "SBE ERR" =>                {sbe_err_vec.push(timestamp);},
                        "SError detected" =>        {serror_vec.push(timestamp);},
                        "CPU Memory Error" =>       {cpu_mem_vec.push(timestamp);},
                        "Machine Check Error" =>    {cce_machine_vec.push(timestamp);},
                        "GPU L2" =>                 {gpu_l2_vec.push(timestamp);},
                        "generated a mmu fault" =>  {mmu_fault_vec.push(timestamp);},
                        "SDHCI_INT_DATA_TIMEOUT" => {flash_write_vec.push(timestamp);},
                        "Timeout waiting for hardware interrupt" => {flash_read_vec.push(timestamp);},
                        "watchdog detected" =>      {watchdog_detected_vec.push(timestamp);},
                        _ =>                         continue, // default case
                    }
                    // println!("All errors: {}", all_errors_vec.len());
                }
            }
        }
        else if our_string.eq("Lost dmesg process"){
            // match logdaemon_handle.unwrap().join() {
            //     Ok(_) => {println!("Logdaemon exited");},
            //     Err(e) => {println!("error attempting to wait on log_daemon: ");}, // is 'return' better than 'break'?
            // }
        }
        else {
            // This works, but needs a way to not double-count all of the timestamps. 
            // TODO: Make a cleanup function. 

            println!("Lost log_daemon. Relaunching...");
            // match dmesg_child.try_wait() {
            //     Ok(Some(status)) => {println!("exited with: {}", status);             // respawn log_daemon with a fresh sender, which will restart dmesg 
            //                             // logdaemon_handle.join();
            //                             // logdaemon_handle = thread::spawn(move || {
            //                             // log_daemon::startup(sender, &mut dmesg_child); // a separate thread launches this. 
            //                             // })
            //                         },             // respawn log_daemon with a fresh sender, which will restart dmesg 
            //     Ok(None) => break,
            //     Err(e) =>           {println!("error attempting to wait: {}", e);  
            //                             // logdaemon_handle.join(); 
            //                             // logdaemon_handle = thread::spawn(move || {
            //                             // log_daemon::startup(sender, &mut dmesg_child); // a separate thread launches this. 
            //                             // });
            //                         break;}, // is 'return' better than 'break'?
            // }

        }
        // If child dies, kill this loop and tell main that we lost the process.
        // match dmesg_child.try_wait() {
        //     Ok(Some(status)) => {println!("exited with: {}", status); dmesg_sender.try_send("Lost dmesg process".to_owned()).unwrap(); break;},
        //     Ok(None) => break,
        //     Err(e) => {println!("error attempting to wait: {}", e);  dmesg_sender.try_send("Lost dmesg process".to_owned()).unwrap(); break;}, // is 'return' better than 'break'?
        // }
    }

        //start watchdog daemon
}