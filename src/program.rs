use std::io::{Read, Write};

use crate::args::Args;

async pub fn run<R: Read, W: Write>(reader: R, mut writer: W, args: Args) {

    println!("{:?}{}", fetchWeather().await.unwrap());

    if let Some(place) = args.place {
        writeln!(writer, "Hello from {}!", place).unwrap();
    } else {
        writeln!(writer, "Okay cool I'm just gonna fetch from your current location").unwrap();
    }

}

async fn fetchWeather() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://wttr.in/").await?;
    response.text().await
}



#[cfg(test)]
mod tests {}
