#!/bin/sh

binary=./target/debug/builtin_false

if [ $0 = "./testsuite.sh" ]
then # We're in the wrong directory
    cd ..
fi

if [ ! -f $binary -a ! -x $binary ]
then
    cargo build
    if [ $? -ne 0 ]
    then
        echo "Compilation failed" >&2
        exit 1
    fi
fi

$binary
exit_status=$?

if [ $exit_status -eq 1 ]
then
    echo "Test 'returns_1': OK"
    exit
else
    echo "Test 'returns_1': KO" >&2
    echo "Returned $exit_status instead of 1" >&2
    exit 1
fi
