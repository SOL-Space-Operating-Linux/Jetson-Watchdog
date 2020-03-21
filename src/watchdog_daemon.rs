extern crate difference;
use std::process::Command;
use std::io::{self, Write};
use difference::{Difference, Changeset};



//static mut &String lshw_old_output = "";
//static mut &String ps_old_output = "";
//static mut check_num::<u32> = 0x0000;


fn run_watchdog_loop() {


    //set global variable to 0x1111

    //Look for kerenel errors/ SEE traces (log_daemon pipe)

    //Check hw against baseline

    //Check software against baseline

    //kick dog
}


pub fn get_ps_output() -> String {

    let mut ps_cmd = Command::new("sh");
    let ps_output = ps_cmd.arg("-c")
        .arg("ps axo pid,comm,ruser,pri,rtprio,stat,stackp,vsize,rss")
        .output()
        .expect("process failed to execute");
    
    return String::from_utf8(ps_output.stdout).unwrap();
}

pub fn process_ps_output(ps_output_old: String, ps_output_new: String) {

    //check ps_output_new for the whitelist

    //compare against previous list and add changeset to file

    let Changeset { diffs, .. } = Changeset::new(&ps_output_old, &ps_output_new, "\n");


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

fn get_lshw_output() {

    let mut lshw_cmd = Command::new("sh")
        .arg("-c")
        .arg("lshw")
        .output()
        .expect("process failed to execute");         
}

fn process_lshw_output(lshw_old_output: &String, lshw_new_output: &String) {
    //compare against boot hardware list

    //
}