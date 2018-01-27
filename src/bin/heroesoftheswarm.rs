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
extern crate env_logger;
extern crate heroesoftheswarm;

use heroesoftheswarm::server;

fn main() {
    // Initialize the logger
    env_logger::init();
    // Initialize a server
    //let game_server = server::GameServer::new("127.0.0.1", 5977, 1000.0, 1000.0, 1);
    //TODO: change this once server is an object
    server::run();
}
