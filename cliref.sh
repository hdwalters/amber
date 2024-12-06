#!/usr/bin/env bash

# Parse command line or print help text.
command=""
files=()
title=""
items=()
depth=0
ultra=0
verbose=0
getopt=$(getopt --options=t:i:d:uv --longoptions=title:,items:,depth:,ultra,verbose,help -- "$@") || exit
eval set -- $getopt
while true; do
    case "$1" in
    -t|--title)
        title="$2"
        shift
        shift
        ;;
    -i|--items)
        items+=("$2")
        shift
        shift
        ;;
    -d|--depth)
        depth="$2"
        shift
        shift
        ;;
    -u|--ultra)
        ultra=1
        shift
        ;;
    -v|--verbose)
        verbose=1
        shift
        ;;
    --help)
        cat <<EOF
Test Amber command line parser
Syntax: $(basename $0) [options]
  COMMAND: Text ........ Command name
  FILES: [Text] ........ File paths
  -t|--title: Text ..... Title text
  -i|--items: [Text] ... Item text
  -d|--depth: Num ...... File depth
  -u|--ultra: Bool ..... Ultra flag
  -v|--verbose: Bool ... Verbose flag
  --help ............... Show help text
EOF
        exit 1
        ;;
    --)
        shift
        break
        ;;
    *)
        exit 1
        ;;
    esac
done

# Copy positional parameters.
command="$1"
shift
files=("$@")

# Print positional parameters and options.
echo "Command: \"${command}\""
for file in "${files[@]}"; do
    echo "File: \"${file}\""
done
echo "Title: \"${title}\""
for item in "${items[@]}"; do
    echo "Item: \"${item}\""
done
echo "Depth: ${depth}"
echo "Ultra: ${ultra}"
echo "Verbose: ${verbose}"
