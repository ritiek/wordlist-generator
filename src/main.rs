extern crate clap;

use clap::{Arg, App};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;


fn factorial(value: u32) -> u32 {
    if value == 0 { 1 }
    else { value * factorial(value-1) }
}


fn generate(n: u32, vchars: &Vec<char>, olength: u32, tlength: u32) -> String {
    let mut seed = String::new();
    for alphabet in n.to_string().chars() {
        seed += &alphabet.to_string();
    }
    seed
}


fn main() {
    let schars: &str = "123abc";
    let mut vchars: Vec<char> = schars.chars().collect();
    let textfile = "text.txt";
    let olength = 5;
    let mut file = File::create(textfile);

    let tlength = vchars.len() as u32;
    vchars.sort();
    let tcombinations = factorial(tlength)/factorial(tlength-olength);
    let mut textfile = OpenOptions::new()
                .write(true)
                .append(true)
                .open(textfile)
                .unwrap();

    for n in 0..tcombinations {
        let seed = generate(n, &vchars, olength, tlength);
        println!("{}", seed);
        //writeln!(textfile, seed);
    }
}
