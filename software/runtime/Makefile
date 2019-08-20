
include ../include/generated/variables.mak

REAL_MISOC_DIR:=${MISOC_DIRECTORY}
# hack to use local software dir
MISOC_DIRECTORY=$(LIBUNWIND_DIRECTORY)/../..

include $(REAL_MISOC_DIR)/software/common.mak

LDFLAGS += --eh-frame-hdr \
	-L../libm \
	-L../libprintf \
	-L../libunwind \
	--start-group

RUSTFLAGS += -Cpanic=unwind

all:: runtime.elf

.PHONY: $(RUSTOUT)/libruntime.a
$(RUSTOUT)/libruntime.a:
	$(cargo) --manifest-path $(RUNTIME_DIRECTORY)/Cargo.toml


$(RUSTOUT)/r_table.o: $(RUNTIME_DIRECTORY)/r_table.c
	$(compile)


runtime.elf: $(RUSTOUT)/libruntime.a $(RUSTOUT)/r_table.o
	$(link) -T $(RUNTIME_DIRECTORY)/linker.ld -lm  -lprintf-float -lunwind-bare --target2=abs


