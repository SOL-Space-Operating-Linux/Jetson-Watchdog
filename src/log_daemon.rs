extern crate logwatcher;
extern crate regex;
use logwatcher::LogWatcher;
use regex::Regex;
//for preexisting logs
use std::env;
use std::fs; //accessing files
use std::vec; //vectors


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
    
    // FIXME: Collapse all of the counters into a length determination of the vectors. 
    let mut sbe_err_counter = 0;
    let mut sbe_err_vec = Vec::new(); // initialize empty vector

    let mut serror_counter = 0;
    let mut serror_vec = Vec::new(); // initialize empty vector

    let mut cpu_mem_error_counter = 0;
    let mut cpu_mem_vec = Vec::new(); // initialize empty vector

    let mut cce_machine_error_counter = 0;
    let mut cce_machine_vec = Vec::new(); // initialize empty vector

    let mut gpu_l2_error_counter = 0;
    let mut gpu_l2_vec = Vec::new(); // initialize empty vector

    let mut mmu_fault_error_counter = 0;
    let mut mmu_fault_vec = Vec::new(); // initialize empty vector
    
    let mut flash_write_error_counter = 0;
    let mut flash_write_vec = Vec::new(); // initialize empty vector
    
    let mut flash_read_error_counter = 0; 
    let mut flash_read_vec = Vec::new(); // initialize empty vector

    let mut watchdog_detected_error_counter = 0;
    let mut watchdog_detected_vec = Vec::new(); // initialize empty vector

    for cap in re.captures_iter(&contents) {

        // println!("Found: {}", &cap[0]);   //whole expression from timestamp to error
        // println!("Timestamp: {}", &cap[1]); // timestamp only
        // println!("Error: {}\n", &cap[2]); // error only

        let mut error_type = cap.get(2).unwrap().as_str();
        let mut timestamp = cap.get(1).unwrap().as_str(); // FIXME: can we process this as a string?



        match error_type { // switch-case statement for processing each error

            "SBE ERR" =>                {sbe_err_counter += 1;
                                         sbe_err_vec.push(timestamp);},
            "SError detected" =>        {serror_counter += 1;
                                         serror_vec.push(timestamp);},
            "CPU Memory Error" =>       {cpu_mem_error_counter += 1;
                                         cpu_mem_vec.push(timestamp);},
            "Machine Check Error" =>    {cce_machine_error_counter += 1;
                                         cce_machine_vec.push(timestamp);},
            "GPU L2" =>                 {gpu_l2_error_counter += 1;
                                         gpu_l2_vec.push(timestamp);},
            "generated a mmu fault" =>  {mmu_fault_error_counter += 1;
                                         mmu_fault_vec.push(timestamp);},
            "SDHCI_INT_DATA_TIMEOUT" => {flash_write_error_counter += 1;
                                         flash_write_vec.push(timestamp);},
            "Timeout waiting for hardware interrupt" => {flash_read_error_counter += 1;
                                        flash_read_vec.push(timestamp);},
            "watchdog detected" =>      {watchdog_detected_error_counter += 1;
                                         watchdog_detected_vec.push(timestamp);},
            _ =>                         continue, // default case

        }

    }
    println!("SBE ERR total: {} Length of vector: {}", sbe_err_counter, sbe_err_vec.len());
    // println!("Length of sbe_err_vector: {}", sbe_err_vec.len());
    // println!("Contents of sbe_err_vector: ");
    // for x in &sbe_err_vec {
    //     println!("{}", x);
    // }        // this was test code to make sure that the vector loaded correctly
    println!("Serror total: {} Length of vector: {}", serror_counter, serror_vec.len());
    println!("CPU Memory Error total: {} Length of vector: {}", cpu_mem_error_counter, cpu_mem_vec.len());
    println!("CCE Machine Check Error total: {} Length of vector: {}", cce_machine_error_counter, cce_machine_vec.len());
    println!("GPU L2 Error total: {} Length of vector: {}", gpu_l2_error_counter, gpu_l2_vec.len());
    println!("MMU Fault Error Counter: {} Length of vector: {}", mmu_fault_error_counter, mmu_fault_vec.len());
    println!("Flash Write Error total: {} Length of vector: {}", flash_write_error_counter, flash_write_vec.len());
    println!("Flash Read Error total: {} Length of vector: {}", flash_read_error_counter, flash_read_vec.len());
    println!("Watchdog CPU Error total (detected): {} Length of vector: {}", watchdog_detected_error_counter, watchdog_detected_vec.len());
}
