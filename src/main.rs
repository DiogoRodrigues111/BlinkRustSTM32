#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use fugit::TimerRateU32;
use panic_halt as _;
use stm32f1xx_hal::{
    pac, prelude::*,
    serial::{Config, Serial}, spi::{Mode, Phase, Spi}
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut gpioa = dp.GPIOA.split();
    let mut afio = dp.AFIO.constrain();

    let mut delay = cp.SYST.delay(&clocks);

    // Configura PA5 como saída ( LED )
    let mut led = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);

    // Configura USART1 ( TX: PA9, RX: PA10 )
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10.into_floating_input(&mut gpioa.crh);
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(9_600.bps()),
        &clocks,
    );
    let (mut tx, mut rx) = serial.split();

    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);
    let spi1_new = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut afio.mapr,
        Mode {phase: Phase::CaptureOnFirstTransition, polarity: stm32f1xx_hal::spi::Polarity::IdleLow},
        TimerRateU32::MHz(1u32), clocks);

    loop {
        /* TITLE_JOB: TRANSMIT TX FROM RX */
        let _a = tx.write_str("hello\r");
        // Check if is idle is ready for operating
        if rx.is_idle()
        {
            for _tx_iterator in _a.iter().cloned() {
                for _rx_iterator in rx.read().iter().cloned() {
                    /*** Needed for listen TX transactions ***/
                    rx.listen();
                }
            }
            // Wait for next time TX
            delay.delay_ms(3000u32);
        }
        else {

            /* Operating one by one
            ( RX -> LED_ON | TX -> LED_OFF ) */

            led.set_high();
            delay.delay_ms(1000u32);
            led.set_low();
            delay.delay_ms(1000u32);
        }
    }
}