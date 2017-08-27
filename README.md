# wordlist-generator

A handy wordlist generator.

## Installation

```
cargo install wordlist-generator
```

for superheroes (latest updates):

```
git clone https://github.com/ritiek/wordlist-generator
cd wordlist-generator
cargo install
```

## Usage

Use the command `wordlist` to start using the tool.

```
USAGE:
    wordlist-generator [OPTIONS] <CHARS>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output-length <LENGTH>    Output length of the generated permutations

ARGS:
    <CHARS>    Characters to generate permutations from
```

## Examples

```
$ wordlist abc
abc
acb
bac
bca
cab
cba
```

```
$ wordlist abc123 -o 2
12
13
1a
1b
1c
21
23
2a
2b
2c
31
32
3a
3b
3c
a1
a2
a3
ab
ac
b1
b2
b3
ba
bc
c1
c2
c3
ca
cb
```

```
$ wordlist abaca -o 3
aaa
aab
aac
aba
abc
aca
acb
baa
bac
bca
caa
cab
cba
```

## Contributing

- Found a bug/ have an idea? Feel free to open your ticket in the [issues section](https://github.com/ritiek/wordlist-generator/issues).

- Even better, send a pull request! :smile:

## License

`The MIT License`
