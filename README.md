# PassGen  
PassGen is a simple command line tool to generate pseudo-random passwords
matching a desired set of (simple constraints)

## Usage (as a tool)
```
passg 0.2.0
PassGen is a tool that lets you generate pseudo-random passwords from the command line

USAGE:
    passg [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alpha <alpha>        What kind of alphabetic characters do you want to allow ? (all = 'all', none = 'none',
                               easily distinguished = 'dist' [eg removes O vs 0], lower case = 'lower', upper case =
                               'upper') [default: dist]
    -d, --digit <digit>        What kind of numeric characters do you want to allow ? (all = 'all', none = 'none',
                               easily distinguished = 'dist' [eg removes O vs 0]) [default: dist]
    -l, --length <length>      The length of the password [default: 20]
    -s, --special <special>    What kind of special characters do you want to allow ? (all = 'all', none = 'none', the
                               most common ones = 'basic') [default: basic]
```

## Usage (as a library)
The crate's documentation gives an example on how to generate a random password.
Basically, you will want to do something along these lines: 
```rust
use passg::prelude::*;

let generator = GeneratorBuilder::default()
    .alpha(Alpha::Dist)         // this is the default
    .digit(Digit::Dist)         // this is the default
    .special(Special::Basic)    // this is the default
    .build()
    .expect("This is never going to fail")
```