OUTPUT_FORMAT("elf32-littlearm", "elf32-littlearm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_reset)

SECTIONS
{
  . = 0x00000000;

  .text :
  {
    . = ALIGN(4);
    KEEP(*(.text.startup));
    *(.text .text.*);
    . = ALIGN(4);
  } 

  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } 

  .data : ALIGN(4)
  {
    . = ALIGN(4);
    *(.data .data.*);
    . = ALIGN(4); 
  } 

  .dynamic : { *(.dynamic) } 
  .dynsym : { *(.dynsym) } 
  .dynstr : { *(.dynstr) } 
  .plt : { *(.plt) } 
  .got : { *(.got) } 
  .got.plt : { *(.got.plt) } 
  .hash : { *(.hash) } 

  __edata = .;

  .bss : ALIGN(4)
  {
    . = ALIGN(4);
    __sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4); 
    __ebss = .;
  } 

  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  } 

  .interp : { *(.interp) } 

  . = . + 0x4000;
    . = ALIGN(4); 
  __stack_start = .;

  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}
