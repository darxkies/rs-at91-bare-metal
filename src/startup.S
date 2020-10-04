  .section .text.startup
  .global semihosting_call

_reset:
	BL handler_reset
	B handler_undefined 
	B handler_software_interrupt 
	B handler_prefetch_abort 
	B handler_data_abort 
	.word __edata
	B handler_irq
	B handler_fiq

handler_reset:
	mov r0, lr
	sub r0, #4
	ldr r1, stack_start
	mov sp, r1
	bl low_init
	b .

handler_undefined: 
	b .

handler_software_interrupt: 
	b .

handler_prefetch_abort: 
	b .

handler_data_abort: 
	b .

handler_reserved: 
	b .

handler_irq: 
	b .

handler_fiq: 
	b .

	.thumb_func
semihosting_call:
	SVC 0xAB
	bx lr

stack_start: .word __stack_start
