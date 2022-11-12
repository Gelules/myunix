#!/bin/sh

./false
if [ $? -eq 1 ]
then
    echo "Simple call: OK"
else
    echo "Simple call: KO"
fi
