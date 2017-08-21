extern crate clap;

use clap::{Arg, App};
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;


fn factorial(value: usize) -> usize {
    if value == 0 { 1 }
    else { value * factorial(value-1) }
}


fn generate(n: usize, vchars: &Vec<char>, olength: usize, tlength: usize) -> String {
    let mut seed = String::new();
    for i in 0..olength {
        seed += &vchars[i].to_string();
    }
    while seed.len() < olength {
        seed += &vchars[0].to_string();
    }
    seed
}


fn main() {
    let schars: &str = "123abc";
    let mut vchars: Vec<char> = schars.chars().collect();
    let textfile = "text.txt";
    let olength: usize = 5;
    let mut file = File::create(textfile);

    let tlength = vchars.len();
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
