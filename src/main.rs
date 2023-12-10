extern crate gpio;
#[macro_use] extern crate rocket;

use gpio::{GpioIn, GpioOut, GpioValue};
use std::{thread, time};
use gpio::sysfs::{SysFsGpioInput, SysFsGpioOutput};

use rocket::Request;


// PINOUT
// https://pi4j.com/1.2/pins/model-3b-plus-rev1.html
// Relay example code
// https://github.com/prichardsondev/raspberrypi-server

// Relay Plugs:
// GPIO_ONE should be our HVAC booster
// GPIO_TWO should be our ultrasonic fogger
// Our UVC lamp should be normally on
// Our rPI should be always on running this

// The relay contains 1 always on and 3 variable pin based on two GPIO states
const GPIO_PIN_ONE : u16 = 17;
const GPIO_PIN_TWO : u16 = 27;

static ENABLED: bool = false;

#[get("/enabled")]
fn enabled() -> String {
    format!("Enabled: {}", ENABLED)
}

fn enable(gpio_in : SysFsGpioInput, gpio_out : SysFsGpioOutput, enabled : bool) {
    // Our HVAC booster and fogger should be enabled at the same time
    let mut gpio_in = SysFsGpioInput::open(GPIO_PIN_ONE).unwrap();
    let mut gpio_out = SysFsGpioOutput::open(GPIO_PIN_TWO).unwrap();
    gpio_out.set_value(enabled).expect("could not set gpio_out");
}

// fn main() {
//     rocket::build().register("/enabled", catchers![not_found]);

    // let mut gpio_in = SysFsGpioInput::open(GPIO_PIN_ONE).unwrap();


    // thread::spawn(move || loop {
    //     let gpio_val : GpioValue = gpio_in.read_value().unwrap();
    //     println!("gpio_in: {:?}", gpio_val);
    //     // TODO: Periodically check the value and set our static used by our server
    // });
    
    // // Print the value of one
    // loop {
    //     println!("gpio_in: {:?}", ENABLED);
    //     thread::sleep(time::Duration::from_millis(100));
    // }
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    // Requires super user
    let mut gpio_in = SysFsGpioInput::open(GPIO_PIN_ONE).unwrap();

    thread::spawn(move || loop {
        let gpio_val : GpioValue = gpio_in.read_value().unwrap();
        println!("gpio_in: {:?}", gpio_val);
        // TODO: Periodically check the value and set our static used by our server
        thread::sleep(time::Duration::from_millis(5000));
    });

    let _rocket = rocket::build()
        .mount("/", routes![enabled])
        .launch()
        .await?;

    Ok(())
}