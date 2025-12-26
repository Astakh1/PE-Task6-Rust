#!/bin/bash

echo "##teamcity[testSuiteStarted name='Rust Tests']"

test_output=$(cargo test --release --test integration_tests -- --format=terse 2>&1)

while IFS= read -r line
do
    if [[ "$line" =~ ^test\ ([^ ]+)\ \.\.\.\ (ok|FAILED)$ ]]; then
        test_name="${BASH_REMATCH[1]}"
        status="${BASH_REMATCH[2]}"
        
        echo "##teamcity[testStarted name='$test_name']"
        
        if [[ "$status" == "FAILED" ]]; then
            echo "##teamcity[testFailed name='$test_name']"
        fi
        
        echo "##teamcity[testFinished name='$test_name']"
    fi
done <<< "$test_output"

echo "##teamcity[testSuiteFinished name='Rust Tests']"