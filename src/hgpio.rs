// PINOUT
// https://pi4j.com/1.2/pins/model-3b-plus-rev1.html
// Relay example code
// https://github.com/prichardsondev/raspberrypi-server

// Relay Plugs:
// GPIO_ONE should be our HVAC booster
// GPIO_TWO should be our ultrasonic fogger
// Our UVC lamp should be normally on
// Our rPI should be always on running this

use gpio::{GpioIn, GpioOut, GpioValue};
use gpio::sysfs::{SysFsGpioInput, SysFsGpioOutput};

// The relay contains 1 always on and 3 variable pin based on two GPIO states
const GPIO_PIN_ONE : u16 = 17;
const GPIO_PIN_TWO : u16 = 27;

pub static ENABLED: bool = false;

pub fn enable(gpio_in : SysFsGpioInput, gpio_out : SysFsGpioOutput, enabled : bool) {
    // Our HVAC booster and fogger should be enabled at the same time
    let mut gpio_in = SysFsGpioInput::open(GPIO_PIN_ONE).unwrap();
    let mut gpio_out = SysFsGpioOutput::open(GPIO_PIN_TWO).unwrap();
    gpio_out.set_value(enabled).expect("could not set gpio_out");
}