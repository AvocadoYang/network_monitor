#!/bin/bash

if [ "$EUID" -ne 0 ]; then
  echo "Permission denied"
  exit 1 
fi

./target/x86_64-unknown-linux-gnu/release/network_monitor