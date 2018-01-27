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
use std::f32;

/// The maximum number of commands that can exist in a swarm program
const MAX_NUM_COMMANDS: usize = 20;

/// Represents a single command in the swarm language
// TODO: Fully design this language
#[derive(Clone, Copy, Debug, PartialEq)]
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

        let command: Vec<&str> = s.trim().split(" ").collect();

        // Match
        match &command[0] {
            &"MOVE" => Ok(SwarmCommand::MOVE), // Move command case
            &"NOOP" => Ok(SwarmCommand::NOOP), // Noop command case
            &"TURN" => if command.len() == 2
            // Check if turn parameter was provided
            {
                match command[1].parse::<f32>() {
                    Ok(val) => if val.is_normal() {
                        Ok(SwarmCommand::TURN(val))
                    }
                    // If parameter is valid, return from function
                    else {
                        Err(GenericError {
                            description: "Invalid float parameter for TURN.".into(),
                        })
                    }, // If parameter is not normal, throw error
                    Err(_) => Err(GenericError {
                        description: "Invalid float parameter for TURN.".into(),
                    }), // If parameter cannot be converted to float, throw error
                }
            } else {
                Err(GenericError {
                    description: "No parameters found for TURN.".into(),
                })
            }, // No parameter provided
            _ => Err(GenericError {
                description: "Command not recognized.".into(),
            }), // Invalid command case
        }
    }
}

/// Test the string conversion command
#[test]
fn test_verifier() {
    let c1: SwarmCommand = match "NOOP".parse() {
        Ok(com1) => com1,
        Err(error) => panic!("Error encountered: {}", error),
    };

    let c2: SwarmCommand = match "MOVE".parse() {
        Ok(com2) => com2,
        Err(error) => panic!("Error encountered: {}", error),
    };

    let c3: SwarmCommand = match "TURN 3.1A".parse() {
        Ok(com3) => com3,
        Err(error) => panic!("Error encountered: {}", error),
    };

    assert_eq!(c1, SwarmCommand::NOOP);
    assert_eq!(c2, SwarmCommand::MOVE);
    assert_eq!(c3, SwarmCommand::TURN(3.14));
}

/// A swarm program is a list of swarm commands
#[derive(Clone, Debug)]
pub struct SwarmProgram {
    /// The list of commands
    pub commands: [SwarmCommand; MAX_NUM_COMMANDS],

    /// Program counter pointing to current command
    pub program_counter: usize,
}

/// Some functions for SwarmProgram
impl SwarmProgram {
    /// Constructor (empty)
    pub fn new() -> Self {
        SwarmProgram {
            commands: [SwarmCommand::NOOP; MAX_NUM_COMMANDS],
            program_counter: 0,
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
