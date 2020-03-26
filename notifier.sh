#!/bin/bash

hostname="https://notifier.lesurpaul.fr"

function lesurp_notify() {
    formatted="$@"
    curl --data \"${formatted}\" --header "Content-Type:application/json" ${hostname}/add 
}

function lesurp_clear() {
    curl -L ${hostname}/clear
}

function lesurp_see() {
    curl -L ${hostname}
}
