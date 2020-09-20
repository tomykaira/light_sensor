/*
#![no_main]
#![no_std]

use panic_halt as _;

use core::mem::MaybeUninit;
use cortex_m_rt::entry;
use pac::interrupt;
use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::{pac, prelude::*};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use cortex_m::asm::wfi;

// These two are owned by the ISR. main() may only access them during the initialization phase,
// where the interrupt is not yet enabled (i.e. no concurrent accesses can occur).
// After enabling the interrupt, main() may not have any references to these objects any more.
// For the sake of minimalism, we do not use RTFM here, which would be the better way.
static mut LED: MaybeUninit<stm32f1xx_hal::gpio::gpioc::PC13<Output<PushPull>>> =
    MaybeUninit::uninit();
static mut INT_PIN: MaybeUninit<stm32f1xx_hal::gpio::gpioa::PA7<Input<Floating>>> =
    MaybeUninit::uninit();

#[interrupt]
fn EXTI9_5() {
    let led = unsafe { &mut *LED.as_mut_ptr() };
    let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };

    if int_pin.check_interrupt() {
        if int_pin.is_high().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }

        // if we don't clear this bit, the ISR would trigger indefinitely
        int_pin.clear_interrupt_pending_bit();
    }
}

#[entry]
fn main() -> ! {
    // initialization phase
    let p = pac::Peripherals::take().unwrap();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();
    {
        // the scope ensures that the int_pin reference is dropped before the first ISR can be executed.

        let mut rcc = p.RCC.constrain();
        let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
        let mut gpioc = p.GPIOC.split(&mut rcc.apb2);
        let mut afio = p.AFIO.constrain(&mut rcc.apb2);

        let led = unsafe { &mut *LED.as_mut_ptr() };
        *led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

        let int_pin = unsafe { &mut *INT_PIN.as_mut_ptr() };
        *int_pin = gpioa.pa7.into_floating_input(&mut gpioa.crl);
        int_pin.make_interrupt_source(&mut afio);
        int_pin.trigger_on_edge(&p.EXTI, Edge::RISING_FALLING);
        int_pin.enable_interrupt(&p.EXTI);
    } // initialization ends here

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }

    loop {
        wfi();
    }
}
 */
#![no_std]
#![no_main]

use embedded_hal::digital::v2::{InputPin, OutputPin};
// you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;
use rtic::app;
use stm32f1xx_hal::{
    gpio::{ExtiPin, gpioc::PC13, Output, PushPull},
    prelude::*,
};
use stm32f1xx_hal::gpio::{Edge, Floating, Input};
use stm32f1xx_hal::gpio::gpioa::PA7;

#[app(device = stm32f1xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        led: PC13<Output<PushPull>>,
        int_pin: PA7<Input<Floating>>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut rcc = cx.device.RCC.constrain();

        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);

        // Acquire the GPIOx peripheral
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpioc = cx.device.GPIOC.split(&mut rcc.apb2);

        // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the
        // function in order to configure the port. For pins 0-7, crl should be passed instead
        let mut led = gpioc .pc13 .into_push_pull_output(&mut gpioc.crh);
        // Init to OFF.
        led.set_high().unwrap();

        let mut int_pin = gpioa.pa7.into_floating_input(&mut gpioa.crl);

        int_pin.make_interrupt_source(&mut afio);
        int_pin.trigger_on_edge(&cx.device.EXTI, Edge::RISING_FALLING);
        int_pin.enable_interrupt(&cx.device.EXTI);

        // Init the static resources to use them later through RTFM
        init::LateResources { led, int_pin }
    }

    // Optional.
    //
    // https://rtfm.rs/0.5/book/en/by-example/app.html#idle
    // > When no idle function is declared, the runtime sets the SLEEPONEXIT bit and then
    // > sends the microcontroller to sleep after running init.
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = EXTI9_5, priority = 1, resources = [led, int_pin])]
    fn change(cx: change::Context) {
        // Depending on the application, you could want to delegate some of the work done here to
        // the idle task if you want to minimize the latency of interrupts with same priority (if
        // you have any). That could be done with some kind of machine state, etc.

        if cx.resources.int_pin.is_high().unwrap() {
            cx.resources.led.set_high().unwrap();
        } else {
            cx.resources.led.set_low().unwrap();
        }

        // if we don't clear this bit, the ISR would trigger indefinitely
        cx.resources.int_pin.clear_interrupt_pending_bit();
    }
};
