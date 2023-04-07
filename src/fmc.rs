use stm32_fmc::FmcPeripheral;

use crate::gpio::{self, Alternate, PushPull};
use crate::rcc::Clocks;
use crate::stm32::{self, FMC};
use crate::time::Hertz;
use stm32_fmc::{AddressPinSet, PinsSdram, Sdram, SdramChip, SdramPinSet, SdramTargetBank};

macro_rules! pins {
    (FMC: $($pin:ident: [$( $( #[ $pmeta:meta ] )* $inst:ty),*])+) => {
        $(
            $(
                $( #[ $pmeta ] )*
                impl stm32_fmc::$pin for $inst {}
            )*
        )+
    }
}

pub struct Fmc {
    reg: FMC,
    clocks: Clocks,
}

impl Fmc {
    fn new(reg: FMC, clocks: Clocks) -> Fmc {
        Fmc { reg, clocks }
    }

    /// A new SDRAM memory via the Flexible Memory Controller
    pub fn sdram<
        BANK: SdramPinSet,
        ADDR: AddressPinSet,
        PINS: PinsSdram<BANK, ADDR>,
        CHIP: SdramChip,
    >(
        fmc: FMC,
        pins: PINS,
        chip: CHIP,
        clocks: Clocks,
    ) -> Sdram<Fmc, CHIP> {
        let fmc = Self::new(fmc, clocks);
        Sdram::new(fmc, pins, chip)
    }

    /// A new SDRAM memory via the Flexible Memory Controller
    pub fn sdram_unchecked<CHIP: SdramChip, BANK: Into<SdramTargetBank>>(
        reg: FMC,
        bank: BANK,
        chip: CHIP,
        clocks: Clocks,
    ) -> Sdram<Fmc, CHIP> {
        let fmc = Self::new(reg, clocks);
        Sdram::new_unchecked(fmc, bank, chip)
    }
}

unsafe impl FmcPeripheral for Fmc {
    const REGISTERS: *const () = crate::stm32::FMC::ptr() as *const ();

    fn enable(&mut self) {
        let rcc = unsafe { &(*crate::stm32::RCC::ptr()) };
        rcc.ahb3enr.modify(|_, w| w.fmcen().set_bit());
    }

    fn memory_controller_enable(&mut self) {
        // Enable the FSMC memory controller, if required.
        // STM32L496 only has one memory controller, and it's enabled when the FSMC clock is enabled.
    }

    fn source_clock_hz(&self) -> u32 {
        self.clocks.hclk().to_Hz()
    }
}

pins! {
    FMC:
        A0: [ gpio::PF0<Alternate<PushPull, 12>> ]
        A1: [ gpio::PF1<Alternate<PushPull, 12>> ]
        A2: [ gpio::PF2<Alternate<PushPull, 12>> ]
        A3: [ gpio::PF3<Alternate<PushPull, 12>> ]
        A4: [ gpio::PF4<Alternate<PushPull, 12>> ]
        A5: [ gpio::PF5<Alternate<PushPull, 12>> ]
        A6: [ gpio::PF12<Alternate<PushPull, 12>> ]
        A7: [ gpio::PF13<Alternate<PushPull, 12>> ]
        A8: [ gpio::PF14<Alternate<PushPull, 12>> ]
        A9: [ gpio::PF15<Alternate<PushPull, 12>> ]
        A10: [ gpio::PG0<Alternate<PushPull, 12>> ]
        A11: [ gpio::PG1<Alternate<PushPull, 12>> ]

        D0: [ gpio::PD14<Alternate<PushPull, 12>> ]
        D1: [ gpio::PD15<Alternate<PushPull, 12>> ]
        D2: [ gpio::PD0<Alternate<PushPull, 12>> ]
        D3: [ gpio::PD1<Alternate<PushPull, 12>> ]
        D4: [ gpio::PE7<Alternate<PushPull, 12>> ]
        D5: [ gpio::PE8<Alternate<PushPull, 12>> ]
        D6: [ gpio::PE9<Alternate<PushPull, 12>> ]
        D7: [ gpio::PE10<Alternate<PushPull, 12>> ]
        D8: [ gpio::PE11<Alternate<PushPull, 12>> ]
        D9: [ gpio::PE12<Alternate<PushPull, 12>> ]
        D10: [ gpio::PE13<Alternate<PushPull, 12>> ]
        D11: [ gpio::PE14<Alternate<PushPull, 12>> ]
        D12: [ gpio::PE15<Alternate<PushPull, 12>> ]
        D13: [ gpio::PD8<Alternate<PushPull, 12>> ]
        D14: [ gpio::PD9<Alternate<PushPull, 12>> ]
        D15: [ gpio::PD10<Alternate<PushPull, 12>> ]

        NBL0: [ gpio::PE0<Alternate<PushPull, 12>> ]
        NBL1: [ gpio::PE1<Alternate<PushPull, 12>> ]

        NE1: [ gpio::PD7<Alternate<PushPull, 12>> ]
        NE2: [ gpio::PG9<Alternate<PushPull, 12>> ]
        NE3: [ gpio::PG10<Alternate<PushPull, 12>> ]

        NOE: [ gpio::PD4<Alternate<PushPull, 12>> ]
        NWE: [ gpio::PD5<Alternate<PushPull, 12>> ]
        NWAIT: [ gpio::PD6<Alternate<PushPull, 12>> ]

        CLK: [ gpio::PD3<Alternate<PushPull, 12>> ]
}
