extern crate logwatcher;
use logwatcher::LogWatcher;



let mut log_watcher = LogWatcher::register("/var/log/kern.log".to_string()).unwrap();

log_watcher.watch(|line: String| {
    //scream if we get a hit of the magic switch statement
    println!("Line {}", line);
});