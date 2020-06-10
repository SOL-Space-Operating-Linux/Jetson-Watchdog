#!/bin/bash
counter=100
while [ $counter -gt 0 ]
do
   sleep 0.1
   echo "on stdout"
   echo "on stderr" >&2
   counter=$(( $counter - 1 ))
done
exit 0