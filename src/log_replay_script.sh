#!/bin/bash

# Amanda Voegtlin SIE-3 05/2020
# Script which reads a log file from the command line, parses the timestamps at the beginning of each line,
# subtracts the difference in times since the last timestamp, and waits that long to print the current one.  

do_on_exit(){
    echo 'NOTIFICATION: log_replay_script.sh stopped by CTRL-C signal'
    exit 0
}

trap 'do_on_exit' SIGINT

filename="$1" # accept filename of log from command line
REGEXP="[[:digit:]]*\.[[:digit:]]*" # .?[[:digit:]]*\.[[:digit:]]*\]"
firstline=$(head -n 1 $1)

[[ "$firstline" =~ $REGEXP ]]
    firsttime=${BASH_REMATCH} # extract the first timestamp before going into the loop

while IFS= read -r line || [[ -n "$line" ]]; do #  on every line, do this 
    # echo "Text read from file: $line"
    [[ "$line" =~ $REGEXP ]]
         timestamp=${BASH_REMATCH}
        waittime=$(echo "$timestamp-$firsttime" | bc)
        firsttime=${BASH_REMATCH}
        sleep $waittime
        echo $line

done < "$filename"