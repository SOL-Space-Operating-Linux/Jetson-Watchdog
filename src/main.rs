// use std::io::{BufRead, BufReader};
// use std::process::{Command, Stdio};
// use std::thread;

// fn main() {
//     let mut child = Command::new("./simpleinputtest.sh")
//         .stdout(Stdio::piped())
//         .stderr(Stdio::piped())
//         .spawn()
//         .unwrap();

//     let out = BufReader::new(child.stdout.take().unwrap());
//     let err = BufReader::new(child.stderr.take().unwrap());

//     let thread = thread::spawn(move || {
//         err.lines().for_each(|line|
//             println!("err: {}", line.unwrap())
//         );
//     });

//     out.lines().for_each(|line|
//         println!("out: {}", line.unwrap())
//     );

//     thread.join().unwrap();

//     let status = child.wait().unwrap();
//     println!("{}", status);
// }


extern crate difference;
use std::{thread, time};
use std::process::Command;
use std::io::{self, Write};
use difference::{Difference, Changeset};

mod watchdog_daemon;
mod log_daemon;
mod public_logwatcher;

//global check register

fn main() {


    //load config, use flight defaults if no file, or parts missing

    //wait for boot to finish

    //get baseline ps and lshw
    println!("Starting Daemon");
    
 //   watchdog_daemon::start_watchdog_daemon();
    log_daemon::main();

    println!("Daemon Started!!!");
    //check # resets to see aliveness, compare against previous baselines, log diffs
    //update new baseline, # resets

    //create pipe
    //start logging daemon
    //start watchdog daemon

    
    
}