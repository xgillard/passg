//! This module defines the type used to configure and generate passwords
use derive_builder::Builder;
use rand::Rng;

use crate::charsets::{Alpha, CollatingSeq, Digit, Special};

/// This is the structure you'll use to generate a random password
#[derive(Debug, Builder)]
pub struct Generator {
    #[builder(default = "20")]
    length: usize,
    #[builder(default = "Alpha::Dist")]
    alpha: Alpha,
    #[builder(default = "Digit::Dist")]
    digit: Digit,
    #[builder(default = "Special::Basic")]
    special: Special,
}
impl Default for Generator {
    fn default() -> Self {
        Generator {
            length: 20,
            alpha: Alpha::Dist,
            digit: Digit::Dist,
            special: Special::Basic,
        }
    }
}
impl Generator {
    /// Generates a new random password as per specified configuration
    pub fn generate(&self) -> String {
        let charset = self.charset();
        let mut out = String::new();
        for _ in 0..self.length {
            let idx = rand::thread_rng().gen_range(0..charset.len());
            out.push(charset[idx]);
        }
        out
    }
    /// Returns a vector comprising only those characters that can be used to
    /// generate a new password
    fn charset(&self) -> Vec<char> {
        let mut chars = vec![];

        self.alpha
            .characters()
            .iter()
            .copied()
            .for_each(|c| chars.push(c));
        self.digit
            .characters()
            .iter()
            .copied()
            .for_each(|c| chars.push(c));
        self.special
            .characters()
            .iter()
            .copied()
            .for_each(|c| chars.push(c));

        chars
    }
}
