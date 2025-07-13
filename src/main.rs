#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f1xx_hal::{ pac, prelude::* };

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let mut _afio = dp.AFIO.constrain();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.toggle();
        delay.delay_ms(1000u16);
    }
}