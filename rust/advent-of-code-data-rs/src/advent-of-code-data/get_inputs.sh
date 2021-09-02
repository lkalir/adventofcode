#!/bin/bash

get_inputs() {
    DIR=$1
    COOKIE=$2
    START=$3
    STOP=$4
    START_DAY=$5
    STOP_DAY=$6

    for year in $(seq $START $STOP); do
        mkdir -p $DIR/$year
        for day in $(seq $START_DAY $STOP_DAY); do
            curl -H "Cookie: session=$COOKIE" -o $DIR/$year/day$day.txt https://adventofcode.com/$year/day/$day/input
        done
    done
}

usage() {
    echo "Usage: $0 -c [COOKIE] -o [output_dir DEFAULT: $PWD] -y [start year DEFAULT: 2015] -Y [stop year DEFAULT: 2020] -d [start day DEFAULT: 1] -D [stop day DEFAULT: 25]"
    exit 1
}

DIR=$PWD
START=2015
STOP=2020
START_DAY=1
STOP_DAY=25

while getopts ":c:o:y:Y:d:D:" o; do
    case "${o}" in
    c)
        COOKIE=${OPTARG}
        ;;
    o)
        DIR=${OPTARG}
        ;;
    y)
        START=${OPTARG}
        ;;
    Y)
        STOP=${OPTARG}
        ;;
    d)
        START_DAY=${OPTARG}
        ;;
    D)
        STOP_DAY=${OPTARG}
        ;;
    *) 
        usage
        ;;

    esac
done

if [ -z $COOKIE ]; then
    usage
fi

get_inputs $DIR $COOKIE $START $STOP $START_DAY $STOP_DAY

