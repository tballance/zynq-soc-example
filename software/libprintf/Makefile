include ../include/generated/variables.mak

REAL_MISOC_DIR:=${MISOC_DIRECTORY}
# hack to use local software dir
MISOC_DIRECTORY=$(LIBUNWIND_DIRECTORY)/../..

include $(REAL_MISOC_DIR)/software/common.mak

all:: libprintf-float.a libprintf-nofloat.a

libprintf-%.a: ctype.o printf-%.o
	$(archive)

printf-float.o: $(LIBPRINTF_DIRECTORY)/printf.c
	$(compile)

printf-nofloat.o: $(LIBPRINTF_DIRECTORY)/printf.c
	$(compile) -D_PRINTF_NO_FLOAT

%.o: $(LIBPRINTF_DIRECTORY)/%.c
	$(compile)
