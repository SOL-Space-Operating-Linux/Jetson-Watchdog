extern crate logwatcher;
extern crate regex;
use logwatcher::LogWatcher;
use regex::Regex;
//for preexisting logs
use std::env;
use std::fs;


pub fn start_log_daemon() {
// This log watcher is going to be the most useful in an ACTUAL usecase, where the board is under assault. How to test this?
// In working with preexisting logs, we are going to read the log to a string and then parse it. 

    // let mut log_watcher = LogWatcher::register("/var/log/kern.log".to_string()).unwrap();
    // println!("I got here!");
    // log_watcher.watch(|line: String| {
    //     //scream if we get a hit of the magic switch statement
    //     println!("Line {}", line);
    // });

//This reads from a file (currently, a hard-coded file) and parses it for expressions, then counts them.
// FIXME: Make it a command line option, and read from kernel log if not stated
    let contents = fs::read_to_string("/home/voegtak1/a_log.txt")
        .expect("Something went wrong reading the log");
  //  println!("The log reads:\n{}", contents); // this worked, which was our sanity check. 
    println!("log_daemon.rs called successfully"); 
    let re = Regex::new(r"(\[\s?[0-9]+\.+[0-9]+\]) .+ (\w*(SBE ERR|SError detected|CPU Memory Error|CCE Machine Check Error)\w*)").unwrap(); // leading r signifies a raw string
    let mut sbe_err_counter = 0;

    for cap in re.captures_iter(&contents) {

        println!("Found: {}", &cap[0]);   //whole expression from timestamp to error
        println!("Timestamp: {}", &cap[1]); // timestamp only
        println!("Error: {}\n", &cap[2]); // error only

        let mut error_type = cap.get(1).unwrap().as_str();
        if &cap[2] == "SBE ERR" {
            sbe_err_counter += 1;
            println!("Added 1 to sbe_err_counter, new count is {}", sbe_err_counter);
        }
    }
    println!("SBE ERR total: {}", sbe_err_counter);

}

// pub fn parse_log() -> String {

// }