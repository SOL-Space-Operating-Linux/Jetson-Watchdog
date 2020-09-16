#!/bin/bash

# Amanda Voegtlin SIE-3 05/2020
# This script is to simulate radiation damage to a board. It will randomly pick an error 
# and then pick a random number of times to repeat it, spaced out in random milliseconds
# and output that to dmesg on a constant loop until killed. Must be run in sudo.
# 
# USAGE: sudo ./dmesg_bash_script.sh

do_on_exit(){
    echo 'NOTIFICATION: dmesg_bash_script.sh stopped by CTRL-C signal' > /dev/kmsg
    exit 0
}

trap 'do_on_exit' SIGINT

arr[0]="SBE ERR"
arr[1]="SError detected"
arr[2]="CPU Memory Error"
arr[3]="Machine Check Error"
arr[4]="GPU L2"
arr[5]="generated a mmu fault"
arr[6]="SDHCI_INT_DATA_TIMEOUT"
arr[7]="Timeout waiting for hardware interrupt"
arr[8]="watchdog detected"

while true; do 
    error=$(($RANDOM % 9))
    repeaterror=$(($RANDOM % 5))
    for ((i=0; i <= $repeaterror; i++)) 
        do 
            echo ${arr[$error]} > /dev/kmsg
            extraerror=$(($RANDOM % 5)) # Sometimes, we get more errors! 
            if (($extraerror <= 1)); then # 40% of the time
                echo ${arr[$RANDOM % 9]} > /dev/kmsg
            fi
            if (($extraerror == 2)); then # 20% of the time
                echo ${arr[$RANDOM % 9]} > /dev/kmsg
                echo ${arr[$RANDOM % 9]} > /dev/kmsg
                echo ${arr[$RANDOM % 9]} > /dev/kmsg
                echo ${arr[$RANDOM % 9]} > /dev/kmsg
            fi
            sleep .000$[($RANDOM % 10) + 1]s 
            repeaterror=$repeat-1
        done
    sleep $[($RANDOM % 5) + 1]s # wait a few seconds before the next radiation surge hits 
done
