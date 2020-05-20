extern crate daemonize;
extern crate difference;
use std::process::Command;
use std::io::{self, Write};
use difference::{Difference, Changeset};
// FIXME: decide whether to group these imports 
use std::{thread, time};
use std::fs::File;
use std::fs::create_dir;
// end FIXME
use daemonize::Daemonize;
use std::sync::atomic::{AtomicU32, Ordering};
#[path = "log_daemon.rs"]
mod log_daemon;

static GLOBAL_VERIFICATION_WORD: AtomicU32 = AtomicU32::new(0x11111111);
//static mut check_num::<u32> = 0x0000;

pub fn start_watchdog_daemon() {

//create file (and parent directories, if they don't exist) 
//FIXME: Add error handling to these calls
    let stdout = create_dir("/tmp/jetson/"); // only fires if folder doesn't exist, does not create parent directories
    let stdout = File::create("/tmp/jetson/daemon.out").unwrap();
    let stderr = File::create("/tmp/jetson/daemon.err").unwrap();
//end FIXME

    let daemonize = Daemonize::new()
        .pid_file("/tmp/jetson/test.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp/jetson/") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/jetson/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/jetson/daemon.err`.
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => run_watchdog_loop(),
        Err(e) => eprintln!("Error, {}", e),
    }
}

fn run_watchdog_loop() {
// This section does a cursory check to see if a global variable is intact in memory.
    //check if Global word is 0x1111 1111
    if 0x11111111 != GLOBAL_VERIFICATION_WORD.load(Ordering::SeqCst) {
        println!("Register does not match, rebooting")
    }

//Initialize some mutable variables to store outputs for comparison
    let mut lshw_boot_output:String = String::from("");
    let mut ps_boot_output:String = String::from("");

    let mut lshw_old_output:String = String::from("");
    let mut ps_old_output:String = String::from("");

    loop {
        println!("Jetson Watchdog Looping");
// This is an aliveness check for whether a sequential count has proceeded properly
        if 0x11111111 != GLOBAL_VERIFICATION_WORD.load(Ordering::SeqCst) {
            println!("Register does not match 0x1111 1111, rebooting")
        }
        GLOBAL_VERIFICATION_WORD.fetch_add(0x22222222, Ordering::SeqCst);

// Retrieve and store output of PS and LS
        let ps_new_output = get_ps_output();
        let lshw_new_output = get_lshw_output();

        if 0x33333333 != GLOBAL_VERIFICATION_WORD.load(Ordering::SeqCst) {
            println!("Register does not match 0x3333 3333, rebooting")
        }
        GLOBAL_VERIFICATION_WORD.fetch_add(0x44444444, Ordering::SeqCst);

        process_ps_output(&ps_old_output, &ps_new_output);
        process_lshw_output(&lshw_old_output, &lshw_new_output);

        if 0x77777777 != GLOBAL_VERIFICATION_WORD.load(Ordering::SeqCst) {
            println!("Register does not match 0x7777 7777, rebooting")
        }
        GLOBAL_VERIFICATION_WORD.fetch_add(0x88888888, Ordering::SeqCst);

// Wait two seconds and repeat
        thread::sleep(time::Duration::from_millis(2000));

        ps_old_output = ps_new_output;
        lshw_old_output = lshw_new_output;

        if 0xFFFFFFFF != GLOBAL_VERIFICATION_WORD.load(Ordering::SeqCst) {
            println!("Register does not match 0xFFFF FFFF, rebooting")
        }
        GLOBAL_VERIFICATION_WORD.store(0x11111111, Ordering::SeqCst);


//Look for kerenel errors/ SEE traces (log_daemon pipe)

        log_daemon::start_log_daemon();


//Check hw against baseline

//Check software against baseline

//kick dog

    }

}


pub fn get_ps_output() -> String {

    let mut ps_cmd = Command::new("sh");
    let ps_output = ps_cmd.arg("-c")
        .arg("ps axo pid,comm,ruser,pri,rtprio,stat,stackp,vsize,rss | grep -v ps | grep -v systemd-journal | grep -v sh | grep -v grep")
        .output()
        .expect("process failed to execute");
    
    return String::from_utf8(ps_output.stdout).unwrap();
}

pub fn process_ps_output(ps_output_1: &String, ps_output_2: &String) {

    //not all on one line, will need to do semantic processing

    //check ps_output_new for the whitelist

    //compare against previous list and add changeset to file

    let Changeset { diffs, .. } = Changeset::new(ps_output_1, ps_output_2, "\n");


    let mut same_count = 0;
    let mut add_count = 0;
    let mut rem_count = 0;

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                //println!("SAME {}", x);
                same_count = same_count + 1;
            }
            Difference::Add(ref x) => {
                println!("+{}", x);
                add_count = add_count + 1;
            }
            Difference::Rem(ref x) => {
                println!("-{}", x);
                rem_count = rem_count + 1;
            }
        }
    }

    println!("Same lines: {}, New Lines: {}, Removed lines: {}", same_count, add_count, rem_count);

    //
}

pub fn get_lshw_output() -> String {

    let mut lshw_cmd = Command::new("sh");
    let lshw_output = lshw_cmd.arg("-c")
        .arg("lshw")
        .output()
        .expect("process failed to execute");

        
    return String::from_utf8(lshw_output.stdout).unwrap();
}

pub fn process_lshw_output(lshw_output_1: &String, lshw_output_2: &String) {
    //check ps_output_new for the whitelist

    //compare against previous list and add changeset to file

    let Changeset { diffs, .. } = Changeset::new(lshw_output_1, lshw_output_2, "\n");


    let mut same_count = 0;
    let mut add_count = 0;
    let mut rem_count = 0;

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                //println!("SAME {}", x);
                same_count = same_count + 1;
            }
            Difference::Add(ref x) => {
                println!("+{}", x);
                add_count = add_count + 1;
            }
            Difference::Rem(ref x) => {
                println!("-{}", x);
                rem_count = rem_count + 1;
            }
        }
    }

    println!("Same lines: {}, New Lines: {}, Removed lines: {}", same_count, add_count, rem_count);

    //
}
