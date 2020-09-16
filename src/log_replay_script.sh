#!/bin/bash

# Amanda Voegtlin SIE-3 05/2020
# Script which reads a log file from the command line, parses the timestamps at the beginning of each line,
# subtracts the difference in times since the last timestamp, and waits that long to print the current one.  
# 
# USAGE: sudo ./log_replay_script.sh [logfile.txt]


do_on_exit(){
    echo 'NOTIFICATION: log_replay_script.sh stopped by CTRL-C signal'
    exit 0
}

trap 'do_on_exit' SIGINT

filename="$1" # accept filename of log from command line
REGEXP="\[[\s]{0,4}[[:digit:]]{0,15}\.[[:digit:]]{5,15}\]*" # .?[[:digit:]]*\.[[:digit:]]*\]"
REGEXPNOBRACKET="[[:digit:]]{0,15}\.[[:digit:]]{5,15}"
firstline=$(head -n 1 $1)

[[ "$firstline" =~ $REGEXP ]]
    if [[ $firstline =~ $REGEXPNOBRACKET ]]; then
        firsttime=${BASH_REMATCH} 
    else
        firsttime=""
    fi
{
    while IFS= read -r line || [[ -n "$line" ]] 
    do #  on every line, do this 
        [[ "$line" =~ $REGEXP ]]
            if [[ $line =~ $REGEXPNOBRACKET ]]; then
                timestamp=${BASH_REMATCH}
                if [[ -z $firsttime ]]; then 
                    firsttime=$timestamp
                fi
                waittime=$(echo "$timestamp-$firsttime" | bc) # this variable is destroyed by the subshell on every loop due to this pipe
                if [[ -z $waittime ]]; then
                    waittime=0.01
                fi
                firsttime=$timestamp
                sleep ${waittime}s
                # Sed throws an error if it encounters "printk message dropped", but still sends the message 
                echo $line | sed "s/..$timestamp.//g" > /dev/kmsg # this line needs to be stripped of its timestamp before it goes into dmesg
            fi

    done
} < "$filename" 