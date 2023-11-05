#!/bin/bash

# Open openOCD in a new console.
#gnome-terminal -x sh -c "openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -s tcl; bash"

# Change to target directory.
filepath="../target/thumbv8m.main-none-eabihf/release/lpctest"


# Open GDB for Core 0 in this console.
gdb-multiarch $filepath --command=debug.gdb
