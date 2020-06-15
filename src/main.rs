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

    println!("Daemon Started!!!");
    //check # resets to see aliveness, compare against previous baselines, log diffs
    //update new baseline, # resets

    //create pipe
    //start logging daemon
    log_daemon::main();
    //start watchdog daemon

    
    
}