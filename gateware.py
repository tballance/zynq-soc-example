
from migen import *
from misoc.targets.zedboard import BaseSoC, SoCCore
from misoc.integration.builder import *

from misoc.interconnect.csr import *
from migen.genlib.resetsync import AsyncResetSynchronizer
from migen.genlib.cdc import MultiReg

import os
import subprocess

from migen.build.platforms import zedboard


class TestModule(Module, AutoCSR):
    def __init__(self):
        self.clock_domains.cd_sys = ClockDomain()

        self.storage = CSRStorage(32, reset=0x12345678)

        self.counter = CSRStatus(32)
        counter = Signal(32)
        self.sync += counter.eq(counter + 1)
        self.comb += self.counter.status.eq(counter)


class Zedboard(SoCCore):
    def __init__(self):

        platform = zedboard.Platform()
        super().__init__(platform=platform)

        fclk0 = self.ps7.fclk.clk[0]
        self.clock_domains.cd_sys = ClockDomain()
        self.specials += Instance("BUFG", i_I=fclk0, o_O=self.cd_sys.clk)

        self.submodules.test = TestModule()
        self.csr_devices.append("test")

        self.comb += self.test.cd_sys.clk.eq(self.cd_sys.clk)


def main():
    soc = Zedboard()
    builder = Builder(soc)
    builder.software_packages = []

    software_dir = os.path.join(os.getcwd(), "software")

    def add_software_package(name):
        builder.add_software_package(name, 
            os.path.join(software_dir, name))

    add_software_package("libm")
    add_software_package("libprintf")
    add_software_package("libunwind")
    add_software_package("runtime")

    builder.build()

    cmd = [
        "bitstream-fix", 
        os.path.join(builder.output_dir, "gateware", "top.bit"), 
        os.path.join(builder.output_dir, "gateware", "top.bit.bin")
    ]
    
    subprocess.check_call(cmd)    


if __name__ == "__main__":
    main()


