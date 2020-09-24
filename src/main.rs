#![no_std]
#![no_main]

use embedded_hal::digital::v2::{InputPin, OutputPin};
// you can put a breakpoint on `rust_begin_unwind` to catch panics
use panic_halt as _;
use rtic::app;
use stm32f1xx_hal::gpio::gpioa::PA7;
use stm32f1xx_hal::gpio::{Edge, Floating, Input, Alternate};
use stm32f1xx_hal::timer::{Tim2NoRemap, Timer};
use stm32f1xx_hal::{
    gpio::{gpioc::PC13, ExtiPin, Output, PushPull},
    pac::{TIM2},
    prelude::*,
};
use stm32f1xx_hal::pwm::{Channel, Pwm, C1, PwmChannel};

#[app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources<P> {
        led: PC13<Output<PushPull>>,
        int_pin: PA7<Input<Floating>>,
        servo: PwmChannel<TIM2, C1>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Take ownership over the raw flash and rcc devices and convert them into the corresponding
        // HAL structs
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut afio = cx.device.AFIO.constrain(&mut rcc.apb2);

        // Acquire the GPIOx peripheral
        let mut gpioa = cx.device.GPIOA.split(&mut rcc.apb2);
        let mut gpioc = cx.device.GPIOC.split(&mut rcc.apb2);

        let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
        let pins = c1;
        let mut pwm = Timer::tim2(cx.device.TIM2, &clocks, &mut rcc.apb1)
            .pwm::<Tim2NoRemap, _, _, _>(pins, &mut afio.mapr, 50.hz());
        let mut servo = pwm.split();

        // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the
        // function in order to configure the port. For pins 0-7, crl should be passed instead
        let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
        // Init to OFF.
        led.set_high().unwrap();

        let mut int_pin = gpioa.pa7.into_floating_input(&mut gpioa.crl);

        int_pin.make_interrupt_source(&mut afio);
        int_pin.trigger_on_edge(&cx.device.EXTI, Edge::RISING_FALLING);
        int_pin.enable_interrupt(&cx.device.EXTI);

        // Init the static resources to use them later through RTFM
        init::LateResources { led, int_pin, servo }
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

    #[task(binds = EXTI9_5, priority = 1, resources = [led, int_pin, servo])]
    fn change(cx: change::Context) {
        // Depending on the application, you could want to delegate some of the work done here to
        // the idle task if you want to minimize the latency of interrupts with same priority (if
        // you have any). That could be done with some kind of machine state, etc.

        if cx.resources.int_pin.is_high().unwrap() {
            cx.resources.led.set_high().unwrap();

            cx.resources.servo.enable();
            let max = cx.resources.servo.get_max_duty();
            cx.resources.servo.set_duty((max / 100) * 4);
            // cx.resources.servo.disable();
        } else {
            cx.resources.led.set_low().unwrap();

            cx.resources.servo.enable();
            let max = cx.resources.servo.get_max_duty();
            cx.resources.servo.set_duty((max / 100) * 6);
            // cx.resources.servo.disable();
        }

        // if we don't clear this bit, the ISR would trigger indefinitely
        cx.resources.int_pin.clear_interrupt_pending_bit();
    }
};
