use std::error::Error;

use super::*;

mod abs;
mod disp;
mod ern;
mod rn;

#[derive(Debug, PartialEq)]
pub struct AddressingError;

impl Error for AddressingError {}

impl std::fmt::Display for AddressingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let var_name = write!(f, "Occurred addressing error.");
        var_name
    }
}
