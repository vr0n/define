#!/bin/bash

# Bash version of define for prototyping

DICT="$(dirname ${0})/gcide.dict"

search() {
  TERM="${1}"

  sed -n '/^'$TERM'\ /I{p; :loop n; p; /^ *$/q; b loop}' "${DICT}"
}

if [ -z "${1}" ]; then
  printf "Usage: %s <word to define>\n" "${0}"
  exit 1
fi

search "${1}"
