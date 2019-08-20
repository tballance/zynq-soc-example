
.global _boot

.text
.code 32
_boot:

    movw r6, #:lower16:vector_table
    movt r6, #:upper16:vector_table
    MCR p15, 0, r6, c12, c0, 0;
    MCR p15, 0, r6, c12, c0, 1;
    bl main;

vector_table_address:
    .word vector_table

.align 5
vector_table:
    ldr     pc, .Lreset_addr
    ldr     pc, .Lundefined_addr
    ldr     pc, .Lswi_addr
    ldr     pc, .Lprefetch_abort_addr
    ldr     pc, .Ldata_abort_addr
    ldr     pc, .Lreserved_addr
    ldr     pc, .Lirq_addr
    ldr     pc, .Lfiq_addr
    .word 0        


.Lreset_addr:
    .word     reset_handler
.Lundefined_addr:
    .word     exception_handler
.Lswi_addr:
    .word     swi_handler
.Lprefetch_abort_addr:
    .word     exception_handler
.Ldata_abort_addr:
    .word     exception_handler
.Lreserved_addr:
    .word     exception_handler
.Lirq_addr:
    .word     irq_handler
.Lfiq_addr:
    .word     fiq_handler    
