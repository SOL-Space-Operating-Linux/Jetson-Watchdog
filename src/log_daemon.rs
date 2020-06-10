extern crate logwatcher;
extern crate regex;
use logwatcher::LogWatcher;
use regex::Regex;
//for preexisting logs
use std::env;
use std::fs; //accessing files
use std::vec; //vectors


pub fn start_log_daemon() {
// Create regex to detect the
    let re = Regex::new(r"(\[\s+?[0-9]+\.+[0-9]+\]) .+ (\w*(SBE ERR|SError detected|CPU Memory Error|Machine Check Error|GPU L2| generated a mmu fault|SDHCI_INT_DATA_TIMEOUT|Timeout waiting for hardware interrupt|watchdog detected)\w*)").unwrap(); // leading r signifies a raw string
        
    // Create empty, dynamic vectors to store error timestamps in for later processing.
    //     FIXME: Vectors cannot be left to grow without bounds. Create a fixed length and a way of saving results if the length is exceeded.
    // let mut all_errors_vec = Vec::new();
    // let mut sbe_err_vec = Vec::new();
    // let mut serror_vec = Vec::new();
    // let mut cpu_mem_vec = Vec::new(); 
    // let mut cce_machine_vec = Vec::new(); 
    // let mut gpu_l2_vec = Vec::new();
    // let mut mmu_fault_vec = Vec::new(); 
    // let mut flash_write_vec = Vec::new(); 
    // let mut flash_read_vec = Vec::new(); 
    // let mut watchdog_detected_vec = Vec::new(); 

    // Create LogWatcher instance
    let mut log_watcher = LogWatcher::register("/var/log/syslog".to_string()).unwrap();
    println!("I got here!");
    log_watcher.watch(|line: String| {
        //scream if we get a hit of the magic switch statement
        // println!("Line {}", line);
        let mut string_from_line = line.to_string();
        let mut all_errors_vec = Vec::new();
        let mut sbe_err_vec = Vec::new();
        let mut serror_vec = Vec::new();
        let mut cpu_mem_vec = Vec::new(); 
        let mut cce_machine_vec = Vec::new(); 
        let mut gpu_l2_vec = Vec::new();
        let mut mmu_fault_vec = Vec::new(); 
        let mut flash_write_vec = Vec::new(); 
        let mut flash_read_vec = Vec::new(); 
        let mut watchdog_detected_vec = Vec::new(); 

        let re = Regex::new(r"(\[.?[0-9]+\.[0-9]+\])\s(SBE ERR|Serror Detected|CPU Memory Error|Machine Check Error|GPU L2| generated a mmu fault|SDHCI_INT_DATA_TIMEOUT|Timeout waiting for hardware interrupt|watchdog detected)").unwrap();        for cap in re.captures_iter(&string_from_line) {
            println!("Found: {}", &cap[0]);   // whole expression from timestamp to error regex
            println!("Timestamp: {}", &cap[1]); // timestamp only
            println!("Error: {}\n", &cap[2]); // error only

            let mut error_type = cap.get(2).unwrap().as_str();
            let raw_timestamp = cap.get(1).unwrap().as_str().replace("[", "").replace("]", "").replace(" ", "");
            let timestamp = raw_timestamp.parse::<f32>().unwrap(); // FIXME: can we process this as a string?
            println!("Raw timestamp: {}", raw_timestamp);
            // let number_timestamp = timestamp.parse::<i32>().unwrap();
            // save the timestamp on the global errors vector, then according the individual error type
            all_errors_vec.push(timestamp);
            match error_type { // switch-case statement for processing each error
                "SBE ERR" =>                {sbe_err_vec.push(timestamp); println!("********Added to SBE Err");},
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
        }
        // println!("Length of sbe_err_vector: {}", sbe_err_vec.len());
        // sbe_err_vec.sort_by(|a,b| b.partial_cmp(a).unwrap()); // reverse the timestamps to make life easier 
        let mut sbe_iter = sbe_err_vec.iter().enumerate(); // do I need an iterator object?
        // Display error totals to stdout
        println!("SBE ERR total: {}", sbe_err_vec.len());
        println!("Serror total: {}", serror_vec.len());
        println!("CPU Memory Error total: {}", cpu_mem_vec.len());
        println!("CCE Machine Check Error total: {}", cce_machine_vec.len());
        println!("GPU L2 Error total: {}", gpu_l2_vec.len());
        println!("MMU Fault Error Counter: {}", mmu_fault_vec.len());
        println!("Flash Write Error total: {}", flash_write_vec.len());
        println!("Flash Read Error total: {}",flash_read_vec.len());
        println!("Watchdog CPU Error total (detected): {}", watchdog_detected_vec.len());
        println!("All errors: {}", all_errors_vec.len());
    });

    // for cap in re.captures_iter(&something.to_string()) {

    //     let mut error_type = cap.get(2).unwrap().as_str();
    //     let raw_timestamp = cap.get(1).unwrap().as_str().replace("[", "").replace("]", "").replace(" ", "");
    //     let timestamp = raw_timestamp.parse::<f32>().unwrap(); // FIXME: can we process this as a string?
    //     // let number_timestamp = timestamp.parse::<i32>().unwrap();
    //     // save the timestamp on the global errors vector, then according the individual error type
    //     all_errors_vec.push(timestamp);
    //     match error_type { // switch-case statement for processing each error

    //         "SBE ERR" =>                {sbe_err_vec.push(timestamp);},
    //         "SError detected" =>        {serror_vec.push(timestamp);},
    //         "CPU Memory Error" =>       {cpu_mem_vec.push(timestamp);},
    //         "Machine Check Error" =>    {cce_machine_vec.push(timestamp);},
    //         "GPU L2" =>                 {gpu_l2_vec.push(timestamp);},
    //         "generated a mmu fault" =>  {mmu_fault_vec.push(timestamp);},
    //         "SDHCI_INT_DATA_TIMEOUT" => {flash_write_vec.push(timestamp);},
    //         "Timeout waiting for hardware interrupt" => {flash_read_vec.push(timestamp);},
    //         "watchdog detected" =>      {watchdog_detected_vec.push(timestamp);},
    //         _ =>                         continue, // default case
    //     }
    // }

    // println!("Length of sbe_err_vector: {}", sbe_err_vec.len());
    // println!("Contents of sbe_err_vector: ");
    // sbe_err_vec.sort_by(|a,b| b.partial_cmp(a).unwrap()); // reverse the timestamps to make life easier 
    // let mut sbe_iter = sbe_err_vec.iter().enumerate(); // do I need an iterator object?

    // // Display error totals to stdout

    // println!("SBE ERR total: {}", sbe_err_vec.len());
    // println!("Serror total: {}", serror_vec.len());
    // println!("CPU Memory Error total: {}", cpu_mem_vec.len());
    // println!("CCE Machine Check Error total: {}", cce_machine_vec.len());
    // println!("GPU L2 Error total: {}", gpu_l2_vec.len());
    // println!("MMU Fault Error Counter: {}", mmu_fault_vec.len());
    // println!("Flash Write Error total: {}", flash_write_vec.len());
    // println!("Flash Read Error total: {}",flash_read_vec.len());
    // println!("Watchdog CPU Error total (detected): {}", watchdog_detected_vec.len());
    // println!("All errors: {}", all_errors_vec.len());


}
