use std::str::FromStr;

use crate::errors::Error;

/// A collating sequence
pub trait CollatingSeq {
    /// return all characters from the charset
    fn characters(&self) -> &[char];
}

/// Sets related to the alphabetic collating sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alpha {
    /// Both lower and upper case letters ("*", "b", "both")    
    All,
    /// Characters that are easily distinguished (O,l,Z).
    Dist,
    /// Lower case letters only ("l", "lc", "lower" or "lower-case")
    Lower,
    /// Upper case letters only ("u", "uc", "upper" or "upper-case")
    Upper,
    /// No letters
    None,
}
impl FromStr for Alpha {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "*"          => Ok(Self::All),
            "a"          => Ok(Self::All),
            "all"        => Ok(Self::All),
            "any"        => Ok(Self::All),
            "d"          => Ok(Self::Dist),
            "e"          => Ok(Self::Dist),
            "easy"       => Ok(Self::Dist),
            "dist"       => Ok(Self::Dist),
            "l"          => Ok(Self::Lower),
            "lc"         => Ok(Self::Lower),
            "lower"      => Ok(Self::Lower),
            "lower-case" => Ok(Self::Lower),
            "u"          => Ok(Self::Upper),
            "uc"         => Ok(Self::Upper),
            "upper"      => Ok(Self::Upper),
            "upper-case" => Ok(Self::Upper),
            "0"          => Ok(Self::None),
            "n"          => Ok(Self::None),
            "none"       => Ok(Self::None),
            _            => Err(Error::ParseError(s.to_string()))
        }
    }
}
/// Sets related to the numeric collating sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Digit { 
    /// All digits
    All,
    /// Characters that are easily distinguished (excludes 0 and 1, 2).
    Dist,
    /// No digits
    None,
}
impl FromStr for Digit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "d"     => Ok(Self::Dist),
            "e"     => Ok(Self::Dist),
            "easy"  => Ok(Self::Dist),
            "dist"  => Ok(Self::Dist),
            "*"     => Ok(Self::All),
            "a"     => Ok(Self::All),
            "all"   => Ok(Self::All),
            "any"   => Ok(Self::All),
            "0"     => Ok(Self::None),
            "n"     => Ok(Self::None),
            "none"  => Ok(Self::None),
            _       => Err(Error::ParseError(s.to_string()))
        }
    }
}
/// Sets related to the special characters collating sequence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Special {
    /// All special characters
    All, 
    /// Only the most usual special characters
    Basic,
    /// No special characters
    None,
}
impl FromStr for Special {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "b"     => Ok(Self::Basic),
            "basic" => Ok(Self::Basic),
            "*"     => Ok(Self::All),
            "a"     => Ok(Self::All),
            "all"   => Ok(Self::All),
            "any"   => Ok(Self::All),
            "0"     => Ok(Self::None),
            "n"     => Ok(Self::None),
            "none"  => Ok(Self::None),
            _       => Err(Error::ParseError(s.to_string()))
        }
    }
}

/// The empty charset
static NONE: [char; 0] = [];
/// The lowercase alphabetic
static ALPHA_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];
/// The upper case alphabetic
static ALPHA_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];
/// All alphabetic characters
static ALPHA_ALL: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 

    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];
/// Only easily distinguished characters
static ALPHA_DIST: [char; 49] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 
    'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 
    
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 
    'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
];

/// All digits
static DIGIT_ALL: [char; 10] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];
/// Only the digits that are easily distinguished
static DIGIT_DIST: [char; 7] = [
    '3', '4', '5', '6', '7', '8', '9'
];
/// *All* (actually, a small subset) the special characters
static SPECIAL_ALL: [char; 29] = [
    '#', '@', '&', '"', '\'', '(', '§', '!', ')', '-', '_', '¨', '^', '*', '$', 
    '€', '%', '£', '`', '<', '>', '?', ',', '.', ';', ':', '/', '+', '='
];
/// Only the most common (easiest to distinguish) special characters
static SPECIAL_BASIC: [char; 19] = [
    '#', '@', '&', '(', '!', ')', '-', '_', '*', '$', '%', '?', ',', '.', ';', 
    ':', '/', '+', '='
];

impl CollatingSeq for Alpha {
    fn characters(&self) -> &[char] {
        match *self {
            Alpha::All   => &ALPHA_ALL,
            Alpha::Dist  => &ALPHA_DIST,
            Alpha::Lower => &ALPHA_LOWER,
            Alpha::Upper => &ALPHA_UPPER,
            Alpha::None  => &NONE
        }
    }
}
impl CollatingSeq for Digit {
    fn characters(&self) -> &[char] {
        match *self {
            Digit::All   => &DIGIT_ALL,
            Digit::Dist  => &DIGIT_DIST,
            Digit::None  => &NONE
        }
    }
}
impl CollatingSeq for Special {
    fn characters(&self) -> &[char] {
        match *self {
            Special::All   => &SPECIAL_ALL,
            Special::Basic => &SPECIAL_BASIC,
            Special::None  => &NONE
        }
    }
}