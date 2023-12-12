use rocket::Request;

use crate::hgpio;

#[get("/enabled")]
fn enabled() -> String {
    format!("Enabled: {}", hgpio::ENABLED)
}

#[rocket::main]
pub async fn serve() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", routes![enabled])
        .launch()
        .await?;

    Ok(())
}