/* This program spawns a child process that reads from dmesg and puts dmesg lines into a buffer. 
The main thread then reads that buffer and collects timestamps for each of 9 error types into vectors of timestamps.
Known issues:  Child process kill command is UNIX only, no Windows implementation. 

A Voegtlin, SIE-3
JHU APL 6/2020
Supercomputing in Space IRAD
*/

extern crate regex;
use regex::Regex;
use std::fs; //accessing files
// -----------
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use crossbeam_channel::unbounded;
use std::sync::Arc;
use std::process;

pub fn startup (dmesg_sender: crossbeam_channel::Sender<String>, dmesg_child: &mut std::process::Child) {
    // for debugging
    println!("log_daemon's pid is {}", process::id());

    // Create the regex to detect errors of interest in dmesg chatter 
    let re = Regex::new(r"(\[.?[0-9]+\.[0-9]+\])(.*?)(SBE ERR|SError detected|CPU Memory Error|Machine Check Error|GPU L2|generated a mmu fault|SDHCI_INT_DATA_TIMEOUT|Timeout waiting for hardware interrupt|watchdog detected)").unwrap();
    
    // MAIN LOOP: read the child's stdout buffer forever, and process. 
    while let Some(ref mut stdout) = dmesg_child.stdout { // while there is something in child's stdout pipe
        let lines = BufReader::new(stdout).lines();
        for line in lines { 
            let our_string = line.unwrap().to_string(); // string type so we can run it through regex
            for cap in re.captures_iter(&our_string) {
                // println!("LOG_DAEMON Sending: {}", cap.get(0).unwrap().as_str().to_string());
                dmesg_sender.try_send(cap.get(0).unwrap().as_str().to_string()).unwrap(); //either send a message into the channel immediately or return an error if the channel is full or disconnected. The returned error contains the original message.
            }       
        } // end of line processing
        // If child dies, kill this loop and tell main that we lost the process.
        match dmesg_child.try_wait() {
            Ok(Some(status)) => {println!("exited with: {}", status); dmesg_sender.try_send("Lost dmesg process".to_owned()).unwrap(); return;},
            Ok(None) => break,
            Err(e) => {println!("error attempting to wait: {}", e);  dmesg_sender.try_send("Lost dmesg process".to_owned()).unwrap(); return;}, // is 'return' better than 'break'?
        }
    } // end of while

    println!("Dmesg process exited!");
    return;
}