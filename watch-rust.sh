##!/bin/bash

build () {
  clear
  cargo build
  }

# 'reflex' program is required for this.
reflex -r 'src/' -s -- sh -c 'clear && cargo build'





