#![no_std]
#![no_main]

use embedded_hal::digital::v2::{InputPin, OutputPin};
// you can put a breakpoint on `rust_begin_unwind` to catch panics
use cortex_m::peripheral::DWT;
use rtic::app;
use rtic::cyccnt::U32Ext as _;
use stm32f1xx_hal::gpio::gpioa::PA7;
use stm32f1xx_hal::gpio::{Edge, Floating, Input};
use stm32f1xx_hal::pwm::{PwmChannel, C1};
use stm32f1xx_hal::timer::{Tim2NoRemap, Timer};
use stm32f1xx_hal::{
    gpio::{gpioc::PC13, ExtiPin, Output, PushPull},
    pac::TIM2,
    prelude::*,
};

use panic_halt as _;

// use panic_semihosting as _;
// use cortex_m_semihosting::hprintln;

// Standard clock of bluepill is 8MHz.
// Because the servo is 50 Hz, we set 40 ms (2 PWM signals).
const SERVO_OFF_PERIOD: u32 = 320_000; // = 8000 x 40
                                       // Bluepill clock speed is 8MHz.
const CLOCK_SPEED: u32 = 8_000_000; // 8 MHz
                                    // We wait at least this seconds to turn light off.
const LIGHT_OFF_SEC: u16 = 60;

const ON_ANGLE: u16 = 6;
const OFF_ANGLE: u16 = 3;

#[app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources<P> {
        led: PC13<Output<PushPull>>,
        int_pin: PA7<Input<Floating>>,
        servo: PwmChannel<TIM2, C1>,
        #[init(0)]
        seconds_to_off: u16,
    }

    #[init(schedule = [tick_second])]
    fn init(mut cx: init::Context) -> init::LateResources {
        cx.core.DCB.enable_trace();
        DWT::unlock();
        cx.core.DWT.enable_cycle_counter();
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
        let pwm = Timer::tim2(cx.device.TIM2, &clocks, &mut rcc.apb1).pwm::<Tim2NoRemap, _, _, _>(
            pins,
            &mut afio.mapr,
            50.hz(),
        );
        let servo = pwm.split();

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
        init::LateResources {
            led,
            int_pin,
            servo,
        }
    }

    // Optional.
    //
    // https://rtfm.rs/0.5/book/en/by-example/app.html#idle
    // > When no idle function is declared, the runtime sets the SLEEPONEXIT bit and then
    // > sends the microcontroller to sleep after running init.
    #[idle(spawn=[tick_second, turn_light_off])]
    fn idle(cx: idle::Context) -> ! {
        cx.spawn.tick_second().unwrap();
        cx.spawn.turn_light_off().unwrap();
        loop {
            cortex_m::asm::wfi();
        }
    }

    #[task(binds = EXTI9_5, priority = 1, resources = [led, int_pin, servo, seconds_to_off], schedule = [turn_servo_off, tick_second])]
    #[allow(unused_must_use)] // <= We must not call unwrap() on them. They become Err().
    fn change(cx: change::Context) {
        // if we don't clear this bit, the ISR would trigger indefinitely
        cx.resources.int_pin.clear_interrupt_pending_bit();

        // Active low
        if cx.resources.int_pin.is_low().unwrap() {
            if *cx.resources.seconds_to_off == 0 {
                cx.resources.led.set_low().unwrap();
                cx.resources.servo.enable();
                let max = cx.resources.servo.get_max_duty();
                cx.resources.servo.set_duty((max / 100) * ON_ANGLE);
                cx.schedule
                    .turn_servo_off(cx.start + SERVO_OFF_PERIOD.cycles());

                cx.schedule
                    .tick_second(cx.start + CLOCK_SPEED.cycles())
                    .unwrap();
            }

            *cx.resources.seconds_to_off = LIGHT_OFF_SEC;
        }
    }

    #[task(resources = [seconds_to_off], schedule = [tick_second], spawn=[turn_light_off])]
    fn tick_second(cx: tick_second::Context) {
        if *cx.resources.seconds_to_off == 0 {
            cx.spawn.turn_light_off().unwrap();
        } else {
            *cx.resources.seconds_to_off -= 1;
            cx.schedule
                .tick_second(cx.scheduled + CLOCK_SPEED.cycles())
                .unwrap();
        }
    }

    #[task(resources=[servo, led, seconds_to_off], schedule = [turn_servo_off, turn_light_off])]
    #[allow(unused_must_use)] // <= We must not call unwrap() on them. They become Err().
    fn turn_light_off(cx: turn_light_off::Context) {
        cx.resources.led.set_low().unwrap();
        cx.resources.servo.enable();
        let max = cx.resources.servo.get_max_duty();
        cx.resources.servo.set_duty((max / 100) * OFF_ANGLE);
        cx.schedule
            .turn_servo_off(cx.scheduled + SERVO_OFF_PERIOD.cycles());
        *cx.resources.seconds_to_off = 0;
    }

    #[task(resources=[servo, led])]
    fn turn_servo_off(cx: turn_servo_off::Context) {
        cx.resources.led.set_high().unwrap();
        cx.resources.servo.disable();
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn TIM2();
    }
};
