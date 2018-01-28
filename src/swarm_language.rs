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
    /// Swarm fires a bullet in indicated direction
    FIRE,
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

        let command: Vec<&str> = s.trim().split_whitespace().collect();

        if (command.len() == 0) {
            return Err(GenericError {
                description: "Command is white space (should be non-error).".into(),
            });
        };

        // Match
        match command[0].to_uppercase().as_str() {
            "MOVE" => Ok(SwarmCommand::MOVE), // Move command case
            "FIRE" => Ok(SwarmCommand::FIRE), // Fire Command case
            "NOOP" => Ok(SwarmCommand::NOOP), // Noop command case
            "TURN" => {
                if command.len() == 2
                // Check if turn parameter was provided
                {
                    match command[1].parse::<f32>() {
                        Ok(val) => {
                            if val.is_normal() {
                                if (val.abs() <= 30.0_f32) {
                                    Ok(SwarmCommand::TURN(val)) // If value satisfies clamp conditions,
                                } else {
                                    Err(GenericError::new(
                                        "Input parameter float should range from -30.0 to 30.0."
                                            .into(),
                                    )) // Otherwise, throw compilation error
                                }
                            } else {
                                Err(GenericError::new(
                                    "Invalid float parameter for TURN.".into(), // If parameter is not normal, throw error
                                ))
                            }
                        }

                        Err(_) => Err(GenericError::new(
                            "Invalid float parameter for TURN.".into(),
                        )), // If parameter cannot be converted to float, throw error
                    }
                } else {
                    Err(GenericError::new("No parameters found for TURN.".into())) // No parameter provided
                }
            }
            _ => Err(
                GenericError::new("Command not recognized.".into()), // Invalid command case
            ),
        }
    }
}

/* BROKEN IN MERGE
/// Test the string conversion command
#[test]
fn test_verifier() {
    let c1: SwarmCommand = match "noop".parse() {
        Ok(com1) => com1,
        Err(error) => panic!("Error encountered: {}", error),
    };

    let c2: SwarmCommand = match "MOve".parse() {
        Ok(com2) => com2,
        Err(error) => panic!("Error encountered: {}", error),
    };

    let c3: SwarmCommand = match "turn -29.5".parse() {
        Ok(com3) => com3,
        Err(error) => panic!("Error encountered: {}", error),
    };

    assert_eq!(c1, SwarmCommand::NOOP);
    assert_eq!(c2, SwarmCommand::MOVE);
    assert_eq!(c3, SwarmCommand::TURN(-29.5));
} END BROKEN IN MERGE */

/// A swarm program is a list of swarm commands
#[derive(Clone, Debug)]
pub struct SwarmProgram {
    /// The list of commands
    pub commands: Vec<SwarmCommand>,

    /// Program counter pointing to current command
    pub program_counter: usize,
}

/// Some functions for SwarmProgram
impl SwarmProgram {
    /// Constructor (empty)
    pub fn new(commands: Vec<SwarmCommand>) -> Self {
        SwarmProgram {
            commands: commands,
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

        // Vector of SwarmCommands
        let mut command_list: Vec<SwarmCommand> = Vec::new();

        // Turn lines into commands
        for line in s.trim().lines() {
            command_list.push(match line.parse() {
                Ok(comm) => comm, // If the command is valid, add it to the list
                Err(error) => {
                    if line.trim().is_empty() {
                        continue;
                    } else {
                        return Err(error);
                    }
                } // If the command is invalid, throw an error
            });

            // If the command list size is exceeded, throw an error
            if (command_list.len() > MAX_NUM_COMMANDS) {
                return Err(GenericError {
                    description: "Program is too long: use fewer commands.".into(),
                });
            }
        }

        // Return command list
        Ok(SwarmProgram::new(command_list))
    }
}

#[test]
fn test_comlist_generator() {
    let mut program: String = String::new();
    program = "MOVE\nFIRE\nMOVE\nTURN -30.0\nNOOP\nNOOP\n\t \n  \t\n\nMOVE\nFIRE".into(); // String is goofy to test whitespace stripping

    // Generate command list from program
    let command_list: SwarmProgram = match program.parse() {
        Ok(comlist) => comlist,
        Err(error) => panic!("Program failed with error: {}", error),
    };

    // Check if all commands registered correctly
    assert_eq!(command_list.commands[0], SwarmCommand::MOVE);
    assert_eq!(command_list.commands[1], SwarmCommand::FIRE);
    assert_eq!(command_list.commands[2], SwarmCommand::MOVE);
    assert_eq!(command_list.commands[3], SwarmCommand::TURN(-30.0));
    assert_eq!(command_list.commands[4], SwarmCommand::NOOP);
    assert_eq!(command_list.commands[5], SwarmCommand::NOOP);
    assert_eq!(command_list.commands[6], SwarmCommand::MOVE);
    assert_eq!(command_list.commands[7], SwarmCommand::FIRE);
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
