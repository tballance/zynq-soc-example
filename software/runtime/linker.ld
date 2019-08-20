
ENTRY(_boot)


MEMORY
{
  ram (rwx) : ORIGIN = 0x1fe00000 , LENGTH = 0x40000
}

_STACK_SIZE = DEFINED(_STACK_SIZE) ? _STACK_SIZE : 0x2000;
_HEAP_SIZE = DEFINED(_HEAP_SIZE) ? _HEAP_SIZE : 0x2000;


SECTIONS
{

  .text : { *(.text) } 
  .data : { *(.data) } 
  .bss : { *(.bss) }  

  .resource_table : {
      __rtable_start = .;
      KEEP(*(.rtable))
      __rtable_end = .;
  } 

} 
