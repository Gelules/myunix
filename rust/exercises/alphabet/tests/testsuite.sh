#!/bin/sh

if [ -d tests ]
then
    cd tests
fi

target="../target/debug/alphabet"
ref="./ref"

if [ ! -x $target ]
then
    echo "$target does not exist" >&2
    exit 1
fi

./$target > output

cmp $ref output
if [ $? -eq 0 ]
then
    echo OK
else
    echo KO >&2
    rm output
    exit 1
fi

rm output
