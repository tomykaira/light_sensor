# Setup

This repository assumes STM32F103C8T6 board called Bluepill and ST-LINK V2 writer.
I use macOS.

Install required programs [for flash](https://alexbirkett.github.io/microcontroller/2019/03/30/flash_bluepill_using_ST_link.html) and [for build](https://github.com/stm32-rs/stm32f1xx-hal).

```
> st-info --probe
Found 1 stlink programmers
 serial:     402805012612344d314b4e00
 hla-serial: "\x40\x28\x05\x01\x26\x12\x34\x4d\x31\x4b\x4e\x00"
 flash:      0 (pagesize: 0)
 sram:       0
 chipid:     0x0004
```

`flash: 0` looks confusing, but no problem.

```
openocd -V -f interface/stlink-v2.cfg -f target/stm32f1x.cfg
```

At first, `Error: jtag status contains invalid mode value - communication failure` error may be printed.
In that case, press reset button on bluepill keeping openocd in the error status.
OpenOCD polls for connection, and next trial will succeed if reset button is pressed at that time.

```
cargo run --release
```

will start GDB session and stop before the start of program.

# Hardware

Current setup:

- Bluepill
- SG90 servo motor
- PSUP7C-02-NCL-16-1 infra-red motion sensor
- AAAA rechargeable battery (1.2V DC out) x 4
- 0.1uF decoupling capacitor around servo

Servo's PWM line is connected to PA0 (A0 on silk).
Infra-red sensor line is connected to PA7 (A7 on silk).
Infra-red sensor is sensitive to unstable power supply, so connected to regulated 3.3V of bluepill.

# LICENSE

You can use this product under MIT license.

Copyright 2020 tomykaira.

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.