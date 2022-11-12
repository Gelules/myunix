#!/bin/sh

./true
if [ $? -eq 0 ]
then
    echo "Simple call: OK"
else
    echo "Simple call: KO"
fi
