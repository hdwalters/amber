#!/usr/bin/env bash

# https://stackoverflow.com/questions/369758/how-to-trim-whitespace-from-a-bash-variable/3352015#3352015
text="  abc    def    "
text="${text#"${text%%[![:space:]]*}"}"
text="${text%"${text##*[![:space:]]}"}"
echo "Trimmed: [$text]"
