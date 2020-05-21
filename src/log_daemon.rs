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
    let contents = fs::read_to_string("/home/voegtak1/a_log3.txt")
        .expect("Something went wrong reading the log");
  //  println!("The log reads:\n{}", contents); // this worked, which was our sanity check. 
    println!("log_daemon.rs called successfully"); 
    let re = Regex::new(r"(\[\s+?[0-9]+\.+[0-9]+\]) .+ (\w*(SBE ERR|SError detected|CPU Memory Error|Machine Check Error|GPU L2| generated a mmu fault|SDHCI_INT_DATA_TIMEOUT|Timeout waiting for hardware interrupt|watchdog detected)\w*)").unwrap(); // leading r signifies a raw string
    let mut sbe_err_counter = 0;
    let mut serror_counter = 0;
    let mut cpu_mem_error_counter = 0;
    let mut cce_machine_error_counter = 0;
    let mut gpu_l2_error_counter = 0;
    let mut mmu_fault_error_counter = 0;
    let mut flash_write_error_counter = 0;
    let mut flash_read_error_counter = 0; 
    let mut watchdog_detected_error_counter = 0;
    
    for cap in re.captures_iter(&contents) {

        // println!("Found: {}", &cap[0]);   //whole expression from timestamp to error
        // println!("Timestamp: {}", &cap[1]); // timestamp only
        // println!("Error: {}\n", &cap[2]); // error only

        let mut error_type = cap.get(2).unwrap().as_str();
        let mut timestamp = cap.get(1),unwrap().as_str(); // FIXME: can we process this as a string?

        match error_type { // switch-case statement for processing each error

            "SBE ERR" => sbe_err_counter += 1,
            "SError detected" => serror_counter += 1,
            "CPU Memory Error" => cpu_mem_error_counter += 1,
            "Machine Check Error" => cce_machine_error_counter += 1,
            "GPU L2" => gpu_l2_error_counter += 1,
            "generated a mmu fault" => mmu_fault_error_counter += 1,
            "SDHCI_INT_DATA_TIMEOUT" => flash_write_error_counter += 1,
            "Timeout waiting for hardware interrupt" => flash_read_error_counter += 1,
            "watchdog detected" => watchdog_detected_error_counter += 1,
            _ => continue, // default case

        }

    }
    println!("SBE ERR total: {}", sbe_err_counter);
    println!("Serror total: {}", serror_counter);
    println!("CPU Memory Error total: {}", cpu_mem_error_counter);
    println!("CCE Machine Check Error total: {}", cce_machine_error_counter);
    println!("GPU L2 Error total: {}", gpu_l2_error_counter);
    println!("MMU Fault Error Counter: {}", mmu_fault_error_counter);
    println!("Flash Write Error total: {}", flash_write_error_counter);
    println!("Flash Read Error total: {}", flash_read_error_counter);
    println!("Watchdog CPU Error total (detected): {}", watchdog_detected_error_counter);
}

// pub fn parse_log() -> String {

// }