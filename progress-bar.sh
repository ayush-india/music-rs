#!/usr/bin/env bash
duration=100
percentage() { printf "%s%%" $(( ((elapsed)*100)/(duration)*100/100 )); }
clean_line() { printf "\r"; }

for (( elapsed=1; elapsed<=duration; elapsed=elapsed+1 )); do
    percentage
    sleep 1
    clean_line
done
clean_line
