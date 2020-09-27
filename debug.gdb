target remote :3333
set backtrace limit 32
layout regs
break reset_handler
break HardFault
break panic
load
