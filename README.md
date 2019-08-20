# zynq-soc-example

Basic example of misoc port to zynq-zedboard
Works with the following setup:
- python 3.5
- https://github.com/tballance/migen/tree/zynq_support
- https://github.com/tballance/misoc/tree/zynq_support
- https://github.com/tballance/migen-axi
- arm-linux-gnueabihf clang toolchain (I compiled my own)
- migen-axi requires https://github.com/peteut/ramda.py (the version currently on pip is broken for py3.5)

I am using this with linux running on the primary core. The runtime.elf includes a resource table entry so that it can be loaded onto the second core through zynq_remoteproc:
$ cp runtime.elf /lib/firmware/
$ echo runtime.elf > /sys/class/remoteproc0/remoteproc0/firmware
$ echo start > /sys/class/remoteproc0/remoteproc0/state

The bitstream can be loaded using fpga_manager using:
$ cp top.bit.bin /lib/firmware/
$ echo top.bit.bin > /sys/class/fpga_manager/firmware

