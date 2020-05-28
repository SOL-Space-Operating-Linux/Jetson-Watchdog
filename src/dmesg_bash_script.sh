#!/bin/bash

# Amanda Voegtlin SIE-3 05/2020
# This script is to simulate radiation damage to a board. It will randomly pick an error 
# and then pick a random number of times to repeat it, spaced out in random milliseconds
# and output that to dmesg on a constant loop until killed. Must be run in sudo.
arr[0]="SBE ERR"
arr[1]="Serror Detected"
arr[2]="CPU Memory Error"
arr[3]="Machine Check Error"
arr[4]="GPU L2"
arr[5]="generated a mmu fault"
arr[6]="SDHCI_INT_DATA_TIMEOUT"
arr[7]="Timeout waiting for hardware interrupt"
arr[8]="watchdog detected"

while true; do 
    error=$(($RANDOM % 9))
    repeat=$(($RANDOM % 5))
    for ((i=0; i <= $repeat; i++)) 
        do 
            echo ${arr[$error]} > /dev/kmsg
            # echo ${arr[$error]}
            sleep .000$[($RANDOM % 10) + 1]s 
            repeat=$repeat-1
        done
    sleep $[($RANDOM % 5) + 1]s
done