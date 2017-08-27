//extern crate clap;
extern crate itertools;
extern crate permutohedron;

//use clap::{Arg, App};
use itertools::Itertools;
use permutohedron::heap_recursive;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;


fn make_combinations(vchars: Vec<char>, olength: usize) -> Vec<Vec<char>> {
    let preset = (0..vchars.len()).combinations(olength);
    let mut combinations = Vec::new();

    for comb in preset {
        let mut combination = Vec::new();
        for item in comb {
            combination.push(vchars[item]);
        }
        combinations.push(combination);
    }

    combinations
}


fn make_permutations(combinations: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut permutations = Vec::new();
    for mut combination in combinations {
        heap_recursive(&mut combination, |permutation| {
            permutations.push(permutation.to_vec())
        });
    }
    permutations
}


fn main() {
    let schars: &str = "123abc";
    let mut vchars: Vec<char> = schars.chars().collect();
    let olength: usize = 5;

    vchars.sort();
    let combinations = make_combinations(vchars, olength);
    let permutations = make_permutations(combinations);
    for perm in permutations {
        println!("{:?}", perm);
    }
}
