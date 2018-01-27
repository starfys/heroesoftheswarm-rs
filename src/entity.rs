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
        unimplemented!()
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
        self.y += bullet_update_distance * self.direction.to_radians().sin();
        // TODO: Check collision
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn update_swarm() {
        let mut swarm = Swarm::new(0.0, 0.0);
        swarm.update();
    }

    #[test]
    fn update_bullet() {
        let mut bullet = Bullet::new(0, 0.0, 0.0);
        bullet.update();
    }
}
