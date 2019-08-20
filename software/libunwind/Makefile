include ../include/generated/variables.mak

REAL_MISOC_DIR:=${MISOC_DIRECTORY}
# hack to use local software dir
MISOC_DIRECTORY=$(LIBUNWIND_DIRECTORY)/../..

include $(REAL_MISOC_DIR)/software/common.mak


COMMONFLAGS += -integrated-as -I. \
	-I$(MISOC_DIRECTORY)/software/include/dyld/ \
	-I$(MISOC_DIRECTORY)/software/unwinder/include/ \
	-I$(LIBUNWIND_DIRECTORY) \
	-D__ELF__ -D__linux__ \
	-D_LIBUNWIND_NO_HEAP \
	-DNDEBUG

# The libunwind code itself obviously doesn't require EH to work (so avoid
# emitting cleanups that reference the personality funcion), but we need
# to be able to start unwinding from _Unwind_RaiseException et al.
CXXFLAGS += -fno-exceptions -funwind-tables

ifeq ($(CPU),or1k)
all:: libunwind-bare.a libunwind-elf.a
endif
ifeq ($(CPU),cortex-a9)
all:: libunwind-bare.a libunwind-elf.a
endif


libunwind-%.a: UnwindRegistersSave.o UnwindRegistersRestore.o \
		UnwindLevel1-%.o UnwindLevel1-gcc-ext-%.o libunwind-%.o \
		Unwind-EHABI-%.o
	$(archive)

%.o: $(MISOC_DIRECTORY)/software/unwinder/src/%.S
	$(assemble)

%-elf.o: $(MISOC_DIRECTORY)/software/unwinder/src/%.cpp
	$(compilexx)

%-elf.o: $(MISOC_DIRECTORY)/software/unwinder/src/%.c
	$(compile)

%-bare.o: $(MISOC_DIRECTORY)/software/unwinder/src/%.cpp
	$(compilexx) -D_LIBUNWIND_IS_BAREMETAL

%-bare.o: $(MISOC_DIRECTORY)/software/unwinder/src/%.c
	$(compile) -D_LIBUNWIND_IS_BAREMETAL
