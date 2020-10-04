target remote :3333
set backtrace limit 32
monitor arm semihosting enable
layout regs
break reset_handler
break HardFault
break panic
load
