#!/bin/bash

# Check if a day number is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <day-number>"
  exit 1
fi

DAY="day-$(printf "%02d" "$1")"

# Check if the specified day directory exists
if [ ! -d "$DAY" ]; then
  echo "Day $1 does not exist."
  exit 1
fi

# Navigate to the day's directory and run the main program
cd "$DAY" || exit
cargo run