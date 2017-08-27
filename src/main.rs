extern crate itertools;
extern crate permutohedron;
#[macro_use(value_t)]
extern crate clap;

use itertools::Itertools;
use permutohedron::heap_recursive;
use clap::{Arg, App};


fn make_combinations(charlist: Vec<char>, olength: usize) -> Vec<Vec<char>> {
    let preset = (0..charlist.len()).combinations(olength);
    let mut combinations = Vec::new();

    for precomb in preset {
        let mut combination = Vec::new();
        for preitem in precomb {
            combination.push(charlist[preitem]);
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
	let arguments = App::new("wordlist-generator")
                    .version("0.1.0")
                    .author("Ritiek Malhotra <ritiekmalhotra123@gmail.com>")
                    .about("A handy wordlist generator")

                    .arg(Arg::with_name("charlist")
                    .index(1)
                    .short("c")
                    .long("character-list")
                    .value_name("CHARS")
                    .takes_value(true)
                    .required(true)
                    .help("Characters to generate permutations from"))

                    .arg(Arg::with_name("olength")
                    .short("o")
                    .long("output-length")
                    .value_name("LENGTH")
                    .takes_value(true)
                    .help("Output length of the generated permutations"))

                    .get_matches();

    let charlist: &str = arguments.value_of("charlist").unwrap();
    let olength: usize = value_t!(arguments, "olength", usize).unwrap_or(charlist.len());

    let mut charlist: Vec<char> = charlist.chars().collect();
    charlist.sort();

    let combinations = make_combinations(charlist, olength);
    let permutations = make_permutations(combinations);

    for permutation in permutations {
        let stringform: String = permutation.into_iter().collect();
        println!("{}", stringform);
    }
}
