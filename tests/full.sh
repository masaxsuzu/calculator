#!/bin/bash

function run_test () {
    test_case_name=$1
    want=$2
    input=$3
    compiler="target/release/calculator"
    echo "${input}" | "${compiler}" > ./out.s
    gcc -o out out.s
    ./out
    got=$?
    diff <(echo "${want}") <(echo "${got}") || \
    ( echo "[Fail][${test_case_name}]" && exit 1) 
}

function run_test_from_file () {
    test_case_name=$1
    want=$2
    input=$3
    compiler="target/release/calculator"
    "${compiler}" -f ${input} > ./out.s
    gcc -o out out.s
    ./out
    got=$?
    diff <(echo "${want}") <(echo "${got}") || \
    ( echo "[Fail][${test_case_name}]" && exit 1) 
}

# stdin
run_test "Simple1" "1" "1"
run_test "Simple2" "10" "10"
run_test "add" "11" "10+1"
run_test "sub" "12" "21-9"
run_test "mul" "100" "10*10"
run_test "div" "4" "12/3"
run_test "infix1" "26" "2*3+4*5"
run_test "infix4" "4" "(3+5)/2"
run_test "infix2" "15" "5*(9-6)"
run_test "infix3" "47" "5+6*7"
run_test "prefix1" "19" "-1+20"
run_test "prefix2" "0" "-30*2-(-60)"

# file
run_test_from_file "add" "2" "tests/files/add"
