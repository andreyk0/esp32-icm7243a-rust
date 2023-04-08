use esp_idf_hal::gpio;
use esp_idf_hal::gpio::{Level, Output, PinDriver};

use std::{thread::sleep, time::Duration};

use anyhow::Result;

pub struct Display<'d> {
    a0: PinDriver<'d, gpio::Gpio3, Output>,
    a1: PinDriver<'d, gpio::Gpio5, Output>,
    a2: PinDriver<'d, gpio::Gpio7, Output>,

    cswr: PinDriver<'d, gpio::Gpio9, Output>,

    d0: PinDriver<'d, gpio::Gpio39, Output>,
    d1: PinDriver<'d, gpio::Gpio37, Output>,
    d2: PinDriver<'d, gpio::Gpio35, Output>,
    d3: PinDriver<'d, gpio::Gpio33, Output>,
    d4: PinDriver<'d, gpio::Gpio18, Output>,
    d5: PinDriver<'d, gpio::Gpio16, Output>,
}

impl<'d> Display<'d> {
    pub fn new(
        a0: PinDriver<'d, gpio::Gpio3, Output>,
        a1: PinDriver<'d, gpio::Gpio5, Output>,
        a2: PinDriver<'d, gpio::Gpio7, Output>,

        mut cswr: PinDriver<'d, gpio::Gpio9, Output>,

        d0: PinDriver<'d, gpio::Gpio39, Output>,
        d1: PinDriver<'d, gpio::Gpio37, Output>,
        d2: PinDriver<'d, gpio::Gpio35, Output>,
        d3: PinDriver<'d, gpio::Gpio33, Output>,
        d4: PinDriver<'d, gpio::Gpio18, Output>,
        d5: PinDriver<'d, gpio::Gpio16, Output>,
    ) -> Result<Self> {
        cswr.set_high()?;

        Ok(Display {
            a0,
            a1,
            a2,
            cswr,
            d0,
            d1,
            d2,
            d3,
            d4,
            d5,
        })
    }

    pub fn display_char(&mut self, a: usize, c: u8) -> Result<()> {
        self.a0.set_level(lvl(a & 0b1))?;
        self.a1.set_level(lvl(a & 0b10))?;
        self.a2.set_level(lvl(a & 0b100))?;

        self.d0.set_level(lvl(c & 0b1))?;
        self.d1.set_level(lvl(c & 0b10))?;
        self.d2.set_level(lvl(c & 0b100))?;
        self.d3.set_level(lvl(c & 0b1000))?;
        self.d4.set_level(lvl(c & 0b10000))?;
        self.d5.set_level(lvl(c & 0b100000))?;

        sleep(Duration::from_micros(100));
        self.cswr.set_low()?;
        sleep(Duration::from_millis(1));
        self.cswr.set_high()?;
        sleep(Duration::from_millis(1));

        Ok(())
    }
}

#[inline]
fn lvl<N>(x: N) -> Level
where
    N: num::Zero,
{
    if x.is_zero() {
        Level::Low
    } else {
        Level::High
    }
}
