extern crate gpio;
#[macro_use] extern crate rocket;

use std::{thread, time};

mod rest;
mod hgpio;


fn main() {

    // thread::spawn(|| {
    // });
    rest::serve();
}

// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {

//     // Requires super user
//     let mut gpio_in = SysFsGpioInput::open(GPIO_PIN_ONE).unwrap();

//     thread::spawn(move || loop {
//         let gpio_val : GpioValue = gpio_in.read_value().unwrap();
//         println!("gpio_in: {:?}", gpio_val);
//         // TODO: Periodically check the value and set our static used by our server
//         thread::sleep(time::Duration::from_millis(5000));
//     });

//     let _rocket = rocket::build()
//         .mount("/", routes![enabled])
//         .launch()
//         .await?;

//     Ok(())
// }