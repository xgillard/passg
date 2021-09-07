# PassG-lib
PassGen is a simple crate to help you to pseudo-random passwords
matching a desired set of (simple constraints)

## Usage 
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