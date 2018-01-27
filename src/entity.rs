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
use world::World;
use std::f32;

/// The initial size of a swarm
const INITIAL_SWARM_SIZE: usize = 10;
/// The maximum size of a swarm
const MAX_SWARM_SIZE: usize = 20;

/// Represents a player's swarm
#[derive(Clone, Debug, Serialize)]
pub struct Swarm {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Direction the swarm is facing
    pub direction: f32,
    /// Members of the swarm
    pub members: Vec<Option<SwarmMember>>,
    /// Color of the swarm
    pub color: (u8, u8, u8),
    /// Experience gained by the swarm
    pub experience: i64,
    /// Duration of bullets in ticks
    pub bullet_duration: i16,
    /// Program used to execute the swarm
    #[serde(skip_serializing)]
    pub program: SwarmProgram,
}
/// Functions for a swarm
impl Swarm {
    /// Constructor
    pub fn new(x: f32, y: f32, num_members: usize) -> Self {
        Swarm {
            x: x,
            y: y,
            direction: 0.0,
            members: vec![Some(SwarmMember::new()); num_members],
            color: (0, 0, 0),
            experience: 0,
            bullet_duration: 60 * 5, // 60fps * 5 seconds default duration
            program: SwarmProgram::new(vec![
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::MOVE,
                SwarmCommand::TURN(10.0),
                SwarmCommand::FIRE,
                SwarmCommand::FIRE,
                SwarmCommand::FIRE,
                SwarmCommand::FIRE,
                SwarmCommand::FIRE,
            ]),
        }
    }
    /// Supplementary function to add color to a swarm. Typically used with the constructor
    pub fn with_color(mut self, color: (u8, u8, u8)) -> Self {
        self.color = color;
        self
    }
    /// Performs 1 tick
    pub fn update(
        &mut self,
        swarm_id: usize,
        world_width: f32,
        world_height: f32,
        bullets: &mut Vec<Bullet>,
    ) {
        // TODO: put this somewhere else
        let swarm_update_distance: f32 = 10.0;
        if self.program.commands.len() != 0 {
            match self.program.commands[self.program.program_counter] {
                SwarmCommand::MOVE => {
                    // When within EPSILON of edge of the world, bounce off it
                    const EPSILON: f32 = 10.0;
                    if self.x - EPSILON <= 0.0 || self.x + EPSILON >= world_width
                        || self.y - EPSILON <= 0.0
                        || self.y + EPSILON >= world_height
                    {
                        self.direction = -self.direction;
                    }

                    // Update the x and y position
                    self.x += swarm_update_distance * self.direction.to_radians().cos();
                    self.y -= swarm_update_distance * self.direction.to_radians().sin();
                }
                SwarmCommand::FIRE => {
                    self.fire(swarm_id, bullets);
                }
                SwarmCommand::TURN(turn_amt) => {
                    // turn logic
                    self.direction += turn_amt;
                    // Keep direction and program counter within their bounds
                    self.direction %= 360.0;
                }
                SwarmCommand::NOOP => {}
            }

            // Update program_counter to point to next command
            self.program.program_counter += 1;
            self.program.program_counter %= self.program.commands.len();
        }
    }

    pub fn fire(&self, swarm_id: usize, bullets: &mut Vec<Bullet>) {
        // spawn bullet with velocity vector
        for member in &self.members {
            match member {
                &Some(cur_swarm_member) => {
                    let new_bullet: Bullet = Bullet::new(
                        swarm_id,
                        self.x + cur_swarm_member.x,
                        self.y + cur_swarm_member.y,
                        self.direction,
                        self.bullet_duration,
                    );
                    bullets.push(new_bullet);
                }
                &None => {}
            }
        }
    }

    // Calculates the offset for a number of position parameters
    pub fn calculate_offsets(radius: f32) -> Vec<(f32, f32)> {
        // Initialize list with origin offset (0,0)
        let mut offset_list: Vec<(f32, f32)> = Vec::new();
        offset_list.push((0.0, 0.0));

        // Generate other offsets
        for i in 1..4 {
            let shell: f32 = i as f32;

            // Generate i*4 positions for each shell
            for j in 0..(i * 4) {
                let rads: f32 = (j as f32) * ((3.141592654) / (2.0 * shell)); // Calculate angle of current offset
                offset_list.push((shell * radius * (rads.cos()), shell * radius * (rads.sin()))); // Push scaled coordinates onto array
            }
        }

        // Return generated offsets
        offset_list
    }
}

#[test]
fn test_offset_calc() {
    let rad1: f32 = 1.0;
    let rad2: f32 = 2.5;

    let ooflist1: Vec<(f32, f32)> = Swarm::calculate_offsets(rad1);
    let ooflist2: Vec<(f32, f32)> = Swarm::calculate_offsets(rad2);

    println!("Offsets of radius 1.0:");
    for tuple in ooflist1.iter() {
        println!("{:?}", tuple);
    }

    println!("\nOffsets of radius 2.5:");
    for tuple in ooflist2.iter() {
        println!("{:?}", tuple);
    }
}

/// Represents a member of a swarm
#[derive(Clone, Copy, Debug, Serialize)]
pub struct SwarmMember {
    /// X position
    pub x: f32,
    /// Y position
    pub y: f32,
    /// Health
    pub health: i32,
}
/// Functions for SwarmMember
impl SwarmMember {
    pub fn new() -> Self {
        SwarmMember {
            x: 0.0,
            y: 0.0,
            health: 5,
        }
    }
}

/// Represents a bullet
#[derive(Clone, Debug, Serialize)]
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
    /// Duration of bullet in ticks; counts down to 0
    pub duration: i16,
}

/// Functions for a bullet
impl Bullet {
    /// Constructor
    // TODO: add arguments
    pub fn new(owner: usize, x: f32, y: f32, direction: f32, duration: i16) -> Self {
        Bullet {
            owner: owner,
            x: x,
            y: y,
            direction: 0.0,
            duration: duration,
        }
    }
    /// Performs 1 tick
    pub fn update(&mut self) {
        // TODO: put this somewhere else
        let bullet_update_distance: f32 = 1.0;
        // Update the x and y position
        self.x += bullet_update_distance * self.direction.to_radians().cos();
        self.y -= bullet_update_distance * self.direction.to_radians().sin();
        // Update duration by ticks
        self.duration -= 1
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
        let mut test_world: World = World::new(100.0, 100.0);
        let origin_x: f32 = 50.0;
        let origin_y: f32 = 50.0;
        let mut swarm = Swarm::new(origin_x, origin_y, 1);
        swarm.program.commands.clear();
        let move_command: SwarmCommand = SwarmCommand::MOVE;
        let turn_command: SwarmCommand = SwarmCommand::TURN(-45.0);
        // 16 steps to complete move turn pairs at 45 degrees
        let num_steps: usize = 16;
        // append commands to program
        for i in 0..num_steps {
            swarm.program.commands.push(turn_command);
            swarm.program.commands.push(move_command);
            println!("{:?} ", i);
        }

        println!("{:?}", swarm.program.commands);

        // execute commands
        for _ in 0..num_steps {
            swarm.update(
                0,
                test_world.width,
                test_world.height,
                &mut test_world.bullets,
            );
            println!("x: {}, y: {}, dir: {}", swarm.x, swarm.y, swarm.direction);
        }
        assert!(swarm.x - origin_x <= f32::EPSILON);
        assert!(swarm.y - origin_y <= f32::EPSILON);
        assert!(swarm.direction - 0.0 <= f32::EPSILON);
    }

    #[test]
    fn test_world_bounds() {
        let mut test_world: World = World::new(10.0, 10.0);
        let mut swarm = Swarm::new(0.0, 0.0, 1);
        let swarm_id = 0;
        let turn_amt = -100.0;
        swarm.program.commands.push(SwarmCommand::TURN(turn_amt));
        swarm.program.commands.push(SwarmCommand::MOVE);

        for _ in 0..2 {
            swarm.update(
                swarm_id,
                test_world.width,
                test_world.height,
                &mut test_world.bullets,
            );
            println!("x: {}, y: {}, dir: {}", swarm.x, swarm.y, swarm.direction);
        }

        //assert!(swarm.x - 1.0 <= f32::EPSILON);
        //assert!(swarm.x - 0.0 <= f32::EPSILON);
        assert!(swarm.direction - -turn_amt <= f32::EPSILON);
    }

    #[test]
    fn update_bullet() {
        let mut bullet = Bullet::new(0, 0.0, 0.0, 0.0, 5);
        bullet.direction = 90.0;
        bullet.update();
        assert!(bullet.x - 1. <= f32::EPSILON);
        assert!(bullet.y - 0. <= f32::EPSILON);
    }
    #[test]
    fn test_bullets() {
        let mut world: World = World::new(10.0, 10.0);
        let swarm_id: usize = 0;
        world.swarms.insert(swarm_id, Swarm::new(5.0, 5.0, 6));
        // TODO: handle the option better later
        world.swarms.get_mut(&swarm_id).unwrap().program.commands = vec![SwarmCommand::FIRE];

        world.swarms.get_mut(&swarm_id).unwrap().update(
            swarm_id,
            world.width,
            world.height,
            &mut world.bullets,
        );

        assert_eq!(world.bullets.len(), 6);
    }
}
