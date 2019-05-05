#!/bin/bash

function run_test () {
    test_case_name=$1
    want=$2
    input=$3
    bin="target/release/calculator"
    got=$(echo "${input}" | "${bin}")
    diff <(echo "${want}") <(echo "${got}") || \
    ( echo "[Fail][${test_case_name}]" && exit 1) 
}

run_test "Simple" "1 1 +" "1+1"
run_test "Priority" "1 2 + 2 *" "(1+2)*2"
run_test "Divide-by-Zero" "1 0 /" "1/0"
run_test "Include-Parse-Error" "" "122 +"
run_test "Include-New-Line" "1 2 - 2 *" "(1-2)*2
"
