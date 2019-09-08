##!/bin/bash

build () {
  # clear
  cargo build
  }

# 'reflex' program is required for this.
reflex -r '\.rs' cargo build


