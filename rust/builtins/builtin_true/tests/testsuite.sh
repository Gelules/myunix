#!/bin/sh

binary=./target/debug/builtin_true

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

if [ $exit_status -eq 0 ]
then
    echo "Test 'returns_0': OK"
    exit
else
    echo "Test 'returns_0': KO" >&2
    echo "Returned $exit_status instead of 0" >&2
    exit 1
fi
