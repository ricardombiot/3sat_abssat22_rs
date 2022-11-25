#!/bin/bash

if [ -z "$1" ]
  then
    echo "No argument supplied"
fi

mv ./output ./output_$1