duration=5
for (( elapsed=1; elapsed<=duration; elapsed=elapsed+1 )); do
    printf "%s%%" $(( ((elapsed)*100)/(duration)*100/100 ))
    sleep 0.2
    printf "\r"
done
