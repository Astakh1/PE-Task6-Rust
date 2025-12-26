#!/bin/bash

echo "##teamcity[testSuiteStarted name='Rust Tests']"

# Запускаем тесты и получаем вывод
test_output=$(cargo test --release --test integration_tests -- --format=terse 2>&1)

echo "DEBUG: Длина вывода тестов: ${#test_output}" >&2
echo "DEBUG: Первые 100 символов вывода: ${test_output:0:100}" >&2

# Парсим результаты
while IFS= read -r line
do
    echo "DEBUG: Обрабатываем строку: $line" >&2
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

# Выводим детальный лог
echo "=== Детальный вывод тестов ==="
cargo test --release --test integration_tests --verbose