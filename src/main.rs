
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
use std::{thread, time, fs, io::Read};
use std::process::{Command, Stdio};
// use nix::sys::wait::waitpid;
// use nix::unistd::{fork, getpid, getppid, ForkResult};


// USAGE: sudo ~/jetson-watchdog

//global check register
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Error_Timestamps { // internal components of struct are private even if struct is public 
    #[serde(skip_serializing_if="Option::is_none")]
    pub all_errors_vec: Option<Vec<f32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub sbe_err_vec: Option<Vec<f32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub serror_vec: Option<Vec<f32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub cpu_mem_vec: Option<Vec<f32>>, 
    #[serde(skip_serializing_if="Option::is_none")]
    pub cce_machine_vec: Option<Vec<f32>>, 
    #[serde(skip_serializing_if="Option::is_none")]
    pub gpu_l2_vec: Option<Vec<f32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub mmu_fault_vec: Option<Vec<f32>>, 
    #[serde(skip_serializing_if="Option::is_none")]
    pub flash_write_vec: Option<Vec<f32>>, 
    #[serde(skip_serializing_if="Option::is_none")]
    pub flash_read_vec: Option<Vec<f32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub watchdog_detected_vec: Option<Vec<f32>> 
}

// impl Error_Timestamps {
//     pub fn new() -> Error_Timestamps {

//     }
// }



/*------------------ HELPER FUNCTIONS --------------------*/
/*Function that returns the first element of a vector without destroying it*/
fn first<T>(v: &Vec<T>) -> Option<&T> {
    if !v.is_empty() {
        serde::export::Some(v.first().unwrap())
    }
    else {
        return None
    }
}
/*Function that returns the last element of a vector without destroying it*/
fn last<T>(v: &Vec<T>) -> Option<&T> {
    if !v.is_empty() {
        serde::export::Some(v.last().unwrap())
    }
    else {
        return None
    }

}
// fn get_first_and_last<T>(v: &Vec<T>) -> Option<&T> {
//     if !v.is_empty() {
//         v.push(serde::export::Some(first(&v)).unwrap();
//         if v.len().unwrap() != 1 {
//             v.push(serde::export::Some(last(&v).unwrap());
//         }
//     }
//     serde::export::Some(v);

// }
/* Function that merges two structs, with the second struct overriding the first if there's a conflict */
fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
    
        (&mut Value::Object(ref mut a), &Value::Object(ref b)) => {  // If there are objects in both, then clone a's key and send a call to recurse
        
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => { // upon recursing, if a has that key, then it receives a copy of b's 
            // *a = b.clone(); //original
            let mut a_vec: Vec<_> = vec![a.clone()].as_slice().to_vec();
            let mut b_vec: Vec<_> = vec![b.clone()].as_slice().to_vec();
            
            if a.is_array() {
                a_vec = a.as_array().unwrap().to_vec();
            }
            if b.is_array() {
                b_vec = b.as_array().unwrap().to_vec();
            }
            if !b_vec.is_null(){
                a_vec.append(&mut b_vec); //works but not completely
                *a = serde_json::to_value(a_vec).unwrap();
            }
            // *a = serde_json::to_value(a_vec).unwrap();
        }
    }
}
/*Function to be called on a thread that sleeps for one second and then sends a message to the receiver */
fn taskmaster_thread(sender: crossbeam_channel::Sender<String>) {
    let message = "Timer interrupt";
    let one_second = time::Duration::from_secs(1);
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
        let todays_date = 
        format!("{}-{:0>2}-{:0>2}.json", 
            year.to_string(),
            now.month().to_string(),
            now.day().to_string());
        // open today's file, creating it if it doesn't exist
        let file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(todays_date);
// TODO: Output to JSON
        // Open a log file with that name
        // let contents = fs::read_to_string(filename)
        //     .expect("Something went wrong reading the log");
        //  FIXME: add a daily check to make a new one every day?
        //  - use same one each day, have Main bark to dmesg to say it rebooted (should be obvious)

        // Discard oldest log after thirty days
        //  -best way is with linux command: 'find /var/log -name "*.json" -type f -mtime +30 -exec rm -f {} \;'
        // but there is also a way to do this with chrono library's signed_duration_since command, where we can specify older than 30 days
        // and use std::fs::read_dir to list all the files in the output log's directory 
        // see https://doc.rust-lang.org/std/fs/fn.read_dir.html



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
    let mut all_errors_vec: Vec<f32> = Vec::new();
    let mut sbe_err_vec: Vec<f32> = Vec::new();
    let mut serror_vec: Vec<f32> = Vec::new();
    let mut cpu_mem_vec: Vec<f32> = Vec::new(); 
    let mut cce_machine_vec: Vec<f32> = Vec::new(); 
    let mut gpu_l2_vec: Vec<f32> = Vec::new();
    let mut mmu_fault_vec: Vec<f32> = Vec::new(); 
    let mut flash_write_vec: Vec<f32> = Vec::new(); 
    let mut flash_read_vec: Vec<f32> = Vec::new(); 
    let mut watchdog_detected_vec: Vec<f32> = Vec::new(); 
    //Set up channels, send the receiver over to log_daemon to communicate back
    let (s, receiver) = unbounded();

/* This portion of the program creates the dmesg -w thread and then spawns a process, log_daemon, which watches that thread*/
// TODO: This needs to return some value to show that it, or dmesg, died.     
// Set up thread builder; this will let us capture an io::Result if the OS fails to create it
    let dmesg_builder = thread::Builder::new();
    let sender = s.clone(); // clone the sender to send off to the thread
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
    let mut logdaemon_handle = dmesg_builder.spawn(move || {
        log_daemon::startup(sender, &mut dmesg_child); // a separate thread launches this. 
    }).unwrap();
/*-------------End log_daemon section-----------*/

// Create a thread which serves as the heartbeat, sleeping and then periodically sending a message
    let interrupt_builder = thread::Builder::new();
    let interrupt_sender = s.clone(); // clone the sender to send off to the thread
    let taskmaster_timer = interrupt_builder.spawn(move || {
        taskmaster_thread(interrupt_sender);
    }).unwrap();
    // TODO: This needs to return some value to show that it, or dmesg, died. 
    let sys_time = time::SystemTime::now();
    loop { 
        let our_string = receiver.recv().unwrap().to_string(); // this blocks until there is a message on the channel.
        // println!("Main received: {}", our_string);
        if our_string.eq("Timer interrupt") {
            println!("Time interrupt detected!");
            println!("First item in watchdog queue: {:?}", first(&watchdog_detected_vec));
            println!("Last item in watchdog queue: {:?}", last(&watchdog_detected_vec));
            // create a json object using the first and last contents of all of these vectors
            let mut new_json = json!({
                "all_errors_vec": [first(&all_errors_vec), last(&all_errors_vec)],
                "sbe_err_vec": [first(&sbe_err_vec), last(&sbe_err_vec)],
                "serror_vec" : [first(&serror_vec), last(&serror_vec)],
                "cpu_mem_vec": [first(&cpu_mem_vec), last(&cpu_mem_vec)],
                "cce_machine_vec": [first(&cce_machine_vec), last(&cce_machine_vec)],

                "gpu_l2_vec": [first(&gpu_l2_vec), last(&gpu_l2_vec)],

                "mmu_fault_vec": [first(&mmu_fault_vec), last(&mmu_fault_vec)],

                "flash_write_vec": [first(&flash_write_vec), last(&flash_write_vec)],

                "flash_read_vec": [first(&flash_read_vec), last(&flash_read_vec)],

                "watchdog_detected_vec":[first(&watchdog_detected_vec), last(&watchdog_detected_vec)]
            });
            // println!("{:#}", new_json);
            //load the previous json object from a file, if it exists
            let mut file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open("test.json")
                .expect("Unable to open file");

            let mut file_as_string = String::new();
            file.read_to_string(&mut file_as_string).unwrap();
            // If this throws an error, it's because there is nothing in the file

            if file_as_string.is_empty() {
                println!("Empty file!");

            } 
            else{ 
            // call merge() on the two json objects, with the file_json being the first argument, and the new json being the second
                let mut file_json = serde_json::from_str(&file_as_string).unwrap(); 
                println!("JSON in the file was {:#?}", file_json);
                merge(&mut file_json, &new_json);
                new_json = file_json;
            }

            let json: String = serde_json::to_string(&new_json).unwrap();
            // write out to the file-- destroy the old contents?            
            fs::write("test.json", &json).expect("Unable to write to file");
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
        else if !our_string.eq("Lost dmesg process") && !our_string.eq("Timer interrupt"){ 
            for cap in re.captures_iter(&our_string) {
                let error_type = cap.get(3).unwrap().as_str(); // take the third argument of the regex, which is the error message
                let raw_timestamp = cap.get(1).unwrap().as_str().replace("[", "").replace("]", "").replace(" ", ""); // take the timestamp
                let timestamp = raw_timestamp.parse::<f32>().unwrap(); // FIXME: can we process this as a string?

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
                // DEBUG PRINTS: watch the error totals increase
                // println!("SBE ERR total: {}", sbe_err_vec.len());
                // println!("Serror total: {}", serror_vec.len());
                // println!("CPU Memory Error total: {}", cpu_mem_vec.len());
                // println!("CCE Machine Check Error total: {}", cce_machine_vec.len());
                // println!("GPU L2 Error total: {}", gpu_l2_vec.len());
                // println!("MMU Fault Error Counter: {}", mmu_fault_vec.len());
                // println!("Flash Write Error total: {}", flash_write_vec.len());
                // println!("Flash Read Error total: {}",flash_read_vec.len());
                println!("Watchdog CPU Error total (detected): {}", watchdog_detected_vec.len());
                println!("All errors: {}", all_errors_vec.len());
                // println!("{}", serde_json::encode(&watchdog_detected_vec));
                println!("First item in watchdog queue: {}", first(&watchdog_detected_vec).unwrap());
                println!("Last item in watchdog queue: {}", last(&watchdog_detected_vec).unwrap());
            }
    
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
    
    }
        //start watchdog daemon
}