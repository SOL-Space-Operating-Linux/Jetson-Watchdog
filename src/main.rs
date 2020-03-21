extern crate difference;
use std::{thread, time};
use std::process::Command;
use std::io::{self, Write};
use difference::{Difference, Changeset};

mod watchdog_daemon;


//global check register

fn main() {

    //load config, use flight defaults if no file, or parts missing

    //wait for boot to finish

    //get baseline ps and lshw
    
    let ps_output_1 = watchdog_daemon::get_ps_output();

    thread::sleep(time::Duration::from_millis(2000));

    let ps_output_2 = watchdog_daemon::get_ps_output();

    watchdog_daemon::process_ps_output(ps_output_1, ps_output_2);


    //check # resets to see aliveness, compare against previous baselines, log diffs
    //update new baseline, # resets

    //create pipe
    //start logging daemon
    //start watchdog daemon
    
}