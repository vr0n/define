#!/usr/bin/zsh

define

VAL="0"
while [ 1 ]; do
  read -s -t 0.25 -n VAL
  if [[ "${VAL}" == "" ]]; then
    break;
  fi
done
