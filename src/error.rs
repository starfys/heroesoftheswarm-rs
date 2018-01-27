// Copyright 2018 Steven Sheffey
// This file is part of heroesoftheswarm.
//
// heroesoftheswarm is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// heroesoftheswarm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with heroesoftheswarm.  If not, see <http://www.gnu.org/licenses/>.
use std::error::Error;
use std::fmt;

/// A very generic error. Used so we can have an easy type of error to use for
/// traits such as FromStr
#[derive(Debug)]
pub struct GenericError {
    pub description: String,
}

/// Functions for GenericError
impl GenericError {
    /// Constructor
    pub fn new(description: String) -> Self {
        GenericError {
            description: description,
        }
    }
}
/// Allows GenericError to be printed
impl fmt::Display for GenericError {
    /// Writes the error using a formatter
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.description)
    }
}

/// Allows GenericError to be used where an error is wanted
impl Error for GenericError {
    /// Description of the error
    fn description(&self) -> &str {
        &self.description
    }
    /// Underlying cause of the error
    fn cause(&self) -> Option<&Error> {
        None
    }
}
