
extern crate difference;
mod watchdog_daemon;
mod log_daemon; // Our implementation of a dmesg watcher
use chrono::{Datelike, Timelike, Utc};

// USAGE: sudo ~/jetson-watchdog

//global check register

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

    //create pipe
    //start logging daemon
    log_daemon::main();
    //start watchdog daemon

    
    
}