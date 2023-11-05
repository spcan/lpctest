# Access DGB server.
target extended-remote :4444

# Set backtrace limit
set backtrace limit 32

# Reset and halt
monitor reset halt

# Change PC to start point
set $pc = 0x140
