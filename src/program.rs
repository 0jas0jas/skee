use std::io::{Read, Write};

use crate::args::Args;

pub fn run<R: Read, W: Write>(reader: R, mut writer: W, args: Args) {
    println!("Hello world here there's going to be something");
}

#[cfg(test)]
mod tests {}
