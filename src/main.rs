
extern crate difference;
#[macro_use]
extern crate crossbeam_channel;
extern crate regex;
extern crate serde_json;
mod watchdog_daemon;
mod log_daemon; // Our implementation of a dmesg watcher
use serde_json::json;

use regex::Regex;
use crossbeam_channel::unbounded;
use chrono::{Datelike, Timelike, Utc};
use std::{thread, time};
use std::process::{Command, Stdio};
// use nix::sys::wait::waitpid;
// use nix::unistd::{fork, getpid, getppid, ForkResult};


// USAGE: sudo ~/jetson-watchdog

//global check register

// struct Error_Timestamps { // internal components of struct are private even if struct is public 
//     pub all_errors_vec: Vec<f32>,
//     pub sbe_err_vec: Vec<f32>,
//     pub serror_vec: Vec<f32>,
//     pub cpu_mem_vec: Vec<f32>, 
//     pub cce_machine_vec: Vec<f32>, 
//     pub gpu_l2_vec: Vec<f32>,
//     pub mmu_fault_vec: Vec<f32>, 
//     pub flash_write_vec: Vec<f32>, 
//     pub flash_read_vec: Vec<f32>,
//     pub watchdog_detected_vec: Vec<f32> 
// }

// impl Error_Timestamps {
//     pub fn new() -> Error_Timestamps {

//     }
// }

/*Function that returns the first element of a vector without destroying it*/
fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}
fn last<T>(v: &Vec<T>) -> Option<&T> {
    v.last()
}
/*Function to be called on a thread that sleeps for one second and then sends a message to the receiver */
fn taskmaster_thread(sender: crossbeam_channel::Sender<String>) {
    let message = "Timer interrupt";
    let one_second = time::Duration::from_millis(1000);
    loop {
        thread::sleep(one_second);
        sender.try_send(message.to_string()).unwrap();
    }
}

fn main() {

        // Generate, format and assemble the current date string in UTC
        let now = Utc::now();
        let (is_common_era, year) = now.year_ce();
        let my_timestamp = 
            format!("{}-{}-{:0>2}", 
                year.to_string(),
                now.month().to_string(),
                now.day().to_string());
        //for a timestamp that also includes time of open and AM/PM, use code below 
/*                    
        let (is_pm, hour) = now.hour12();
        let my_timestamp = 
            format!("{:0>2}:{:0>2}:{:0>2} {} {}-{}-{:0>2}", 
                hour.to_string(),
                now.minute().to_string(), 
                now.second().to_string(),
                (if is_pm { "PM" } else { "AM" }),
                year.to_string(),
                now.month().to_string(),
                now.day().to_string()); */

// TODO: Output to JSON
        // Open a log file with that name
        // let contents = fs::read_to_string(filename)
        //     .expect("Something went wrong reading the log");
        //  FIXME: add a daily check to make a new one every day?
        //  - use same one each day, have Main bark to dmesg to say it rebooted (should be obvious)

        // Discard oldest log after thirty days
        //  -best way is with linux command: 'find /var/log -name "*.json" -type f -mtime +30 -exec rm -f {} \;'



    //load config, use flight defaults if no file, or parts missing

    //wait for boot to finish

    //get baseline ps and lshw
    println!("Starting Daemon");
    
 //   watchdog_daemon::start_watchdog_daemon();

    println!("Daemon Started!!!");
    //check # resets to see aliveness, compare against previous baselines, log diffs
    //update new baseline, # resets

/* -----------------------------------------LOG DAEMON WORK AREA----------------------------------------------*/
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

// Create a thread which serves as the heartbeat, sleeping and then periodically sending a message
    let interrupt_builder = thread::Builder::new();
    let interrupt_sender = s.clone(); // clone the sender to send off to the thread
    let taskmaster_timer = interrupt_builder.spawn(move || {
        taskmaster_thread(interrupt_sender);
    }).unwrap();

    // Create the regex for local parse and sort use 
    let re = Regex::new(r"(\[.?[0-9]+\.[0-9]+\])(.*?)(SBE ERR|SError detected|CPU Memory Error|Machine Check Error|GPU L2|generated a mmu fault|SDHCI_INT_DATA_TIMEOUT|Timeout waiting for hardware interrupt|watchdog detected)").unwrap();

    // Launch the taskmaster thread, which will tell our main thread when it should analyze data and put it away

    // TODO: This needs to return some value to show that it, or dmesg, died. 
    loop { 
        let our_string = receiver.recv().unwrap().to_string(); // this blocks until there is a message on the channel.
        // println!("Main received: {}", our_string);

        if our_string.eq("Timer interrupt") {
            println!("Time interrupt detected!");
        }
        else if !our_string.eq("Lost dmesg process") && !our_string.eq("Timer interrupt"){ 
            for cap in re.captures_iter(&our_string) {
                // println!("{}", cap.get(1).unwrap().as_str());
                // println!("{}", cap.get(2).unwrap().as_str());
                // println!("{}", cap.get(3).unwrap().as_str());
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



            // When the program times out

            println!("First item in watchdog queue: {}", first(&watchdog_detected_vec).unwrap());
            println!("Last item in watchdog queue: {}", last(&watchdog_detected_vec).unwrap() );
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