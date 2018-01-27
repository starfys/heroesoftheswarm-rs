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
use swarm_language::SwarmProgram;
use swarm_language::SwarmCommand;
use std::f32::{self, consts};

/// The initial size of a swarm
const INITIAL_SWARM_SIZE: usize = 10;
/// The maximum size of a swarm
const MAX_SWARM_SIZE: usize = 20;

/// Represents a player's swarm
#[derive(Clone, Debug)]
pub struct Swarm {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Direction the swarm is facing
    pub direction: f32,
    /// Members of the swarm
    pub members: [Option<SwarmMember>; MAX_SWARM_SIZE],
    /// Color of the swarm
    pub color: (u8, u8, u8),
    /// Program used to execute the swarm
    pub program: SwarmProgram,
}
/// Functions for a swarm
impl Swarm {
    /// Constructor
    pub fn new(x: f32, y: f32) -> Self {
        Swarm {
            x: x,
            y: y,
            direction: 0.0,
            members: [None; MAX_SWARM_SIZE],
            color: (0, 0, 0),
            program: SwarmProgram::new(),
        }
    }
    /// Performs 1 tick
    pub fn update(&mut self) {
        // TODO: put this somewhere else
        let swarm_update_distance: f32 = 1.0;

        let pc: usize = self.program.program_counter;

        match self.program.commands[pc] {
            SwarmCommand::MOVE => {
                // Update the x and y position
                self.x += swarm_update_distance * self.direction.to_radians().cos();
                self.y -= swarm_update_distance * self.direction.to_radians().sin();
            }
            SwarmCommand::TURN(turn_amt) => {
                // turn logic
                self.direction += turn_amt;
            }
            SwarmCommand::NOOP => {
                println!("No operation.");
            }
        }

        self.program.program_counter += 1;

        // TODO: Check collision
        //
        // TODO: Bounds checking
        // maybe do as match?
        self.direction %= 360.0;
        self.program.program_counter %= self.program.commands.len();
    }
}
/// Represents a member of a swarm
#[derive(Clone, Copy, Debug)]
pub struct SwarmMember {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Health
    pub health: i32,
}

/// Represents a bullet
#[derive(Clone, Debug)]
pub struct Bullet {
    /// ID of the player that fired this bullet
    /// This is used so the player can't shoot themself
    pub owner: usize,
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Direction in degrees
    pub direction: f32,
}

/// Functions for a bullet
impl Bullet {
    /// Constructor
    // TODO: add arguments
    pub fn new(owner: usize, x: f32, y: f32) -> Self {
        Bullet {
            owner: owner,
            x: x,
            y: y,
            direction: 0.0,
        }
    }
    /// Performs 1 tick
    pub fn update(&mut self) {
        // TODO: put this somewhere else
        let bullet_update_distance: f32 = 1.0;
        // Update the x and y position
        self.x += bullet_update_distance * self.direction.to_radians().cos();
        self.y -= bullet_update_distance * self.direction.to_radians().sin();
        // TODO: Check collision
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// This test will start at the origin with 0 degrees, move, turn 45 degrees
    /// then move.  This will happen four times, and should return to the original
    /// position with a direction of 0 degrees.
    fn update_swarm() {
        let mut swarm = Swarm::new(0.0, 0.0);
        let move_command: SwarmCommand = SwarmCommand::MOVE;
        let turn_command: SwarmCommand = SwarmCommand::TURN(-45.0);

        // 16 steps to complete move turn pairs at 45 degrees
        let num_steps: usize = 16;
        // append commands to program
        for i in (0..num_steps).step_by(2) {
            swarm.program.commands[i] = move_command;
            swarm.program.commands[i + 1] = turn_command;
            println!("{:?} ", i);
        }

        println!("{:?}\n", swarm.program.commands);


        // execute commands
        for i in (0..num_steps) {
            swarm.update();
            println!("x: {}, y: {}, dir: {}\n", swarm.x, swarm.y, swarm.direction);
        }
        assert!(swarm.x - 0.0 <= f32::EPSILON);
        assert!(swarm.y - 0.0 <= f32::EPSILON);
        assert!(swarm.direction - 0.0 <= f32::EPSILON);
    }

    #[test]
    fn update_bullet() {
        let mut bullet = Bullet::new(0, 0.0, 0.0);
        bullet.direction = 90.0;
        bullet.update();
        assert!(bullet.x - 1. <= f32::EPSILON);
        assert!(bullet.y - 0. <= f32::EPSILON);
    }
}
