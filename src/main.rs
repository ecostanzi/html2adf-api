#[macro_use] extern crate rocket;

use htmltoadf::convert_html_str_to_adf_str;
use rocket::Data;
use std::io;


// #[post("/", format="text/html", data="<data>")]
// fn index(data: Data) -> String  {
//     let body = data.open(10_i32.into()).into_string();
//     let converted = convert_html_str_to_adf_str(body);
//     converted
// }

#[get("/health")]
fn health() -> &'static str {
    "Healthy!"
}

#[post("/convert", format="text/html", data="<data>")]
async fn convert(data: Data<'_>) -> io::Result<String> {
    let string = data.open(2048_i32.into()).into_string().await?;
    if !string.is_complete() {
        println!("there are bytes remaining in the stream");
    }

    Ok(convert_html_str_to_adf_str(string.into_inner()))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, convert])
}