//extern crate clap;
extern crate itertools;
extern crate permutohedron;

//use clap::{Arg, App};
use itertools::Itertools;
use permutohedron::heap_recursive;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;
use collections::slice::Permutations;


fn main() {
    let schars = String::from("123abc");
    //let schars: &str = "123abc";
    //let mut vchars: Vec<char> = schars.chars().collect();
    let textfile = "text.txt";
    let olength: u32 = 5;

    let mut file = File::create(textfile);

    //vchars.sort();
    //let it = (1..5).combinations(3);
    let it = schars.combinations(2);
    let mut textfile = OpenOptions::new()
                .write(true)
                .append(true)
                .open(textfile)
                .unwrap();

    //writeln!(textfile, seed);
}
