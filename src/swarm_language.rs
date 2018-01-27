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
use error::GenericError;
use std::str::FromStr;

/// The maximum number of commands that can exist in a swarm program
const MAX_NUM_COMMANDS: usize = 20;

/// Represents a single command in the swarm language
// TODO: Fully design this language
#[derive(Clone, Copy, Debug)]
pub enum SwarmCommand {
    /// Move the swarm forward
    MOVE,
    /// Rotate the swarm some number of degrees
    TURN(f32),
    /// Do nothing
    NOOP,
}
/// Allows conversion of a string to a command
impl FromStr for SwarmCommand {
    /// The type of error returned if the conversion fails
    /// Must be implemented
    type Err = GenericError;
    /// Converts a string to a SwarmCommand
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Parse a line of swarm code as an enum
        unimplemented!()
    }
}
/// A swarm program is a list of swarm commands
#[derive(Clone, Debug)]
pub struct SwarmProgram {
    /// The list of commands
    pub commands: [SwarmCommand; MAX_NUM_COMMANDS],
}

/// Some functions for SwarmProgram
impl SwarmProgram {
    /// Constructor (empty)
    pub fn new() -> Self {
        SwarmProgram {
            commands: [SwarmCommand::NOOP; MAX_NUM_COMMANDS],
        }
    }
}

/// Allows conversion of a string to a program
impl FromStr for SwarmProgram {
    /// The type of error returned if the conversion fails
    /// Must be implemented
    type Err = GenericError;
    /// Converts a string to a SwarmProgram
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Split the input and use SwarmCommand's from_str
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn parse_swarm_command() {
        let command: SwarmCommand = "test".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_swarm_program() {
        let command: SwarmProgram = "test".parse().unwrap();
    }
}
