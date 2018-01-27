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
extern crate serde_json;
use entity::{Bullet, Swarm};
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::time::{Duration, Instant};
/// Represents the state of the game's world
#[derive(Clone, Debug)]
pub struct World {
    /// The width of the world
    pub width: f32,
    /// The height of the world
    pub height: f32,
    /// Each swarm in the world
    /// Map of player ID to swarm
    pub swarms: HashMap<usize, Swarm>,
    /// Each bullet in the world
    /// TODO: vec and element swap
    pub bullets: Vec<Bullet>,
}
/// Functions for the world
impl World {
    /// Constructor
    /// width: the width of the world
    /// height: the height of the world
    pub fn new(width: f32, height: f32) -> Self {
        World {
            width: width,
            height: height,
            swarms: HashMap::new(),
            bullets: Vec::new(),
        }
    }
    /// Capacity constructor
    /// width: the width of the world
    /// height: the height of the world
    /// capacity: the number players to allocate space for
    /// Space is allocated for 100x the number of bullets
    pub fn with_capacity(width: f32, height: f32, capacity: usize) -> Self {
        World {
            width: width,
            height: height,
            swarms: HashMap::with_capacity(capacity),
            bullets: Vec::with_capacity(capacity * 10),
        }
    }
    /// Adds a player to the server with the given ID
    pub fn add_player(&mut self, id: usize) {
        info!("Adding player {} to the server", id);
        // TODO: determine the initial number of members to make
        let initial_num_members: usize = 10;
        // Get a random position
        let (x, y) = self.random_position();
        // Get a random color
        let color = World::random_color();
        self.swarms
            .insert(id, Swarm::new(x, y, initial_num_members).with_color(color));
    }

    /// Removes a player to the server with the given ID
    pub fn remove_player(&mut self, id: usize) {
        info!("Removing player {} from the server", id);
        match self.swarms.remove(&id) {
            _ => {}
        }
    }
    /// Generates a random position
    fn random_position(&self) -> (f32, f32) {
        // Get the rng
        let mut rng = thread_rng();
        // Defines a margin
        // TODO: make this an associated const
        let margin: f32 = 50.0;
        // Generate the position
        (
            rng.gen_range(margin, self.width - margin),
            rng.gen_range(margin, self.height - margin),
        )
    }
    /// Generates a random color
    fn random_color() -> (u8, u8, u8) {
        // Get the RNG
        let mut rng = thread_rng();
        // Generate the color
        (rng.gen(), rng.gen(), rng.gen())
    }
    /// Performs one "tick" of the world
    /// return: The amount of time elapsed during the tick
    /// Executes each swarm's program on itself
    /// Moves bullets
    /// Does bullet collision
    pub fn update(&mut self) -> Duration {
        // Record time at beginning of update
        let start_time = Instant::now();
        // Update each member of the swarm with its own program
        for (id, swarm) in self.swarms.iter_mut() {
            swarm.update(*id, self.width, self.height, &mut self.bullets);
        }

        // Update each bullet

        let mut i: usize = 0;
        while i < self.bullets.len() {
            // position update bullets

            self.bullets[i].update();

            // remove expired bullets
            if self.bullets[i].duration == 0 {
                self.bullets.swap_remove(i);
                i += 1;
            }

            // collision detection here

            // check each swarm
            for (id, swarm) in self.swarms.iter_mut() {
                // TODO: choose the epsilon to consider as "incoming dangerous
                // bullets"
                let epsilon: f32 = 10.0;
                if self.bullets[i].x - swarm.x <= epsilon && self.bullets[i].y - swarm.y <= epsilon
                {
                    let mut j: usize = 0;
                    while j < swarm.members.len() {
                        // collision detection
                        let swarm_member_radius: f32 = 5.0;

                        // unwrap member
                        match swarm.members[j] {
                            Some(mut member) => {
                                // detect colllision
                                // for now detects if the bullet passes within a
                                // square hitbox around the swarm member
                                if (self.bullets[i].x - member.x).abs() <= swarm_member_radius
                                    && (self.bullets[i].y - member.y).abs() <= swarm_member_radius
                                {
                                    member.health -= 1;

                                    if member.health == 0 {
                                        swarm.members[j] = None;
                                        // increment to next member if member was set to None
                                        j += 1;
                                    }
                                    // delete bullet
                                    self.bullets.swap_remove(i);
                                    i += 1;
                                }
                            }
                            None => {}
                        }
                        // increment to next member
                        j += 1;
                    }
                }
            }
            // increment to next bullet
            i += 1;
        }
        // Record time at end of update and return the time elapsed
        Instant::now().duration_since(start_time)
    }
    /// Returns the world in byte representation
    /// Used to render the world on a client
    pub fn get_state(&self) -> WorldState {
        WorldState {
            swarms: self.swarms.clone(),
            bullets: self.bullets.clone(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct WorldState {
    swarms: HashMap<usize, Swarm>,
    bullets: Vec<Bullet>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize_world() {
        let world = World::new(1000.0, 1000.0);
    }
}
