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

use futures::{Future, Sink, Stream};
use rpc::{CompileRequest, CompileResult, Configuration, Response, ResponseMessage, Vec2};
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::thread;
use std::time::Duration;
use swarm_language::SwarmProgram;
use tokio_core::reactor::{Core, Handle};
use websocket::message::{Message, OwnedMessage};
use websocket::async::Server;
use world::World;

/// Represents a server for the game
// TODO: populate this with parameters
pub struct GameServer {
    // Hostname for the websocket to listen on
    //hostname: String,
    // Port for the websocket to listen on
    //port: u16,
    // Number of server updates per second
    //update_freq: u64,
    // The game's world
    //world: Arc<RwLock<World>>,
    // A counter used to assign player IDs to session
    //id_counter: AtomicUsize,
}
impl GameServer {
    /// Constructor
    /// hostname: address for the websocket to listen on
    /// port: port for the websocket to listen on
    /// width: width of the game world
    /// height: height of the game world
    /// update_freq: server updates per second
    /*pub fn new(hostname: &str, port: u16, width: f32, height: f32, update_freq: u64) -> Self {
        GameServer {
            hostname: hostname.into(),
            port: port,
            update_freq: update_freq,
            world: Arc::new(RwLock::new(World::new(width, height))),
            id_counter: AtomicUsize::new(0),
        }
    }*/
    /// Starts the server
    pub fn start() {}
    /// Handles an incoming websocket message
    fn handle_message(
        message: OwnedMessage,
        player_id: usize,
        world: &Arc<RwLock<World>>,
    ) -> Option<OwnedMessage> {
        match message {
            // Handle incoming text data
            OwnedMessage::Text(data) => {
                // Try to parse it as a request for updates
                match serde_json::from_str::<Vec<Vec2>>(&data) {
                    Ok(coords) => match world.read() {
                        Ok(world) => {
                            // Create a message type
                            let message = Response::new(ResponseMessage::WORLD(world.get_state()));
                            match message.serialize() {
                                Ok(message) => return Some(OwnedMessage::Text(message)),
                                Err(error) => {}
                            }
                        }
                        Err(error) => {
                            warn!("Failed to get read lock on world. Not sending world state");
                            return None;
                        }
                    },
                    Err(_) => debug!("Failed to parse request as a viewport"),
                };
                // Try to parse it as a request for compilation
                match serde_json::from_str::<CompileRequest>(&data) {
                    Ok(compile_request) => match world.write() {
                        Ok(mut write_lock) => {
                            let world_ref = write_lock.deref_mut();
                            info!("Compile request: {}", data);
                            // Create a message type
                            match compile_request.program.parse::<SwarmProgram>() {
                                Ok(compiled_program) => {
                                    // Update the program
                                    world_ref.update_program(player_id, compiled_program);
                                    // Respond with success
                                    let message = Response::new(ResponseMessage::COMPILE(
                                        CompileResult::new(true, "".into()),
                                    ));
                                    match message.serialize() {
                                        Ok(message) => return Some(OwnedMessage::Text(message)),
                                        Err(_) => {}
                                    }
                                }
                                Err(error) => {
                                    info!("Failed to compile program: {}", error);
                                    // Generate an output message
                                    let message = Response::new(ResponseMessage::COMPILE(
                                        CompileResult::new(false, error.to_string()),
                                    ));
                                    match message.serialize() {
                                        Ok(message) => return Some(OwnedMessage::Text(message)),
                                        Err(_) => {}
                                    }
                                }
                            }
                        }
                        Err(error) => {
                            warn!("Failed to get write lock on world. Not updating program");
                            return None;
                        }
                    },
                    Err(_) => debug!("Failed to parse request as a viewport"),
                };
                // If it matches none of the cases, just return None
                None
            }
            // Handle incoming binary data
            OwnedMessage::Binary(_) => None,
            // Handle heartbeats
            OwnedMessage::Ping(p) => Some(OwnedMessage::Pong(p)),
            OwnedMessage::Pong(_) => None,
            // This is technically handled by the take_while function,
            // but it's required to handle
            OwnedMessage::Close(_) => unreachable!(),
        }
    }
}

/// Runs the server
// TODO: Move all of this into impl for GameServer
pub fn run() {
    // Server parameters
    let hostname = "0.0.0.0";
    let port: u16 = 8080;
    let update_freq: u64 = 60;
    // Create the world
    let world: Arc<RwLock<World>> = Arc::new(RwLock::new(World::new(1600.0, 900.0)));
    // Copy a reference to world for the clients to use
    let world_client = world.clone();
    // Start the world's main thread
    thread::spawn(move || {
        // TODO: nanoseconds accuracy for this
        let update_delta = Duration::from_micros(1000000 / update_freq);
        // Get reference to the world
        let world = world.clone();
        // Elapsed time of last update
        let mut last_update_time = Duration::from_millis(0);
        // Main loop
        loop {
            // Log time elapsed in previous update
            debug!(
                "Last update took {}s, {}ns",
                last_update_time.as_secs(),
                last_update_time.subsec_nanos()
            );
            // Sleep for some amount of time
            thread::sleep(update_delta - last_update_time);
            // Lock the world for writing
            match world.write() {
                Ok(mut write_lock) => {
                    // Get a mutable reference to the world
                    let world_ref = write_lock.deref_mut();
                    // Update the world
                    last_update_time = world_ref.update();
                    // Write lock goes out of scope, world is again available to be read
                }
                Err(error) => error!("Error retrieving write lock in update thread: {}", error),
            }
        }
    });
    // Used to assign IDs to connections (players)
    let id_counter: AtomicUsize = AtomicUsize::new(0);
    // Used for serving
    let mut core = Core::new().expect("Failed to initialize core");
    let handle = core.handle();
    // Bind to an address
    let server = Server::bind(format!("{}:{}", hostname, port), &handle)
        .expect("Failed to bind to an address");
    // This future represents what this server is going to do.
    // Handles a stream of incoming connections
    let server_future = server.incoming()
        // Handle errors
        .map(Some)
        .or_else(|_| -> Result<_, ()> { Ok(None) })
        .filter_map(|x| x) 
        //.map_err(move |InvalidConnection { error, .. }| error)
        // Handle connections
        .for_each(move |(upgrade, addr)| {
            // Log the connection
            info!("Got a connection from: {}", addr);
            // Verify protocol
            if !upgrade.protocols().iter().any(|protocol| protocol == "heroesoftheswarm") {
                // Reject connecitons that don't have the supported protocol
                spawn_future(upgrade.reject(), "Upgrade Rejection", &handle);
                return Ok(());
            }
            // Get a reference to the world for this connection
            let world = world_client.clone();
            let w = world.clone();
            // Get an ID for this connection
            let session_id: usize = id_counter.fetch_add(1, AtomicOrdering::SeqCst);
            // Create a swarm for this session
            match world.write() {
                Ok(mut write_lock) => {
                    // Get a mutable reference to the world
                    let world_ref = write_lock.deref_mut();
                    world_ref.add_player(session_id);
                    // Write lock goes out of scope, world is again available to be read
                },
                Err(error) => {
                    error!("Error getting write lock: {}. Player not added", error);
                    spawn_future(upgrade.reject(), "Failed to add player to world", &handle);
                    return Ok(());
                }
            }
            // accept the request to be a ws connection if it does
            let message_handler = upgrade
                // Use our protocol
                .use_protocol("heroesoftheswarm")
                // Accept the message
                .accept()
                // Respond so the client knows the connection succeeded 
                .and_then(move |(socket, _)| {
                    //socket.send(Message::text(session_id.to_string()).into());
                    // Create a config object and send it to the client
                    let config = Configuration::new(session_id); 
                    // Create a response
                    let response = Response::new(ResponseMessage::CONFIG(config));
                    match response.serialize() {
                        Ok(serialized) => socket.send(Message::text(serialized).into()),
                        Err(error) => {
                            error!("Failed to serialize config");
                            socket.send(Message::text(r#"{"mt": "error", "message": {"error": "Failed to serialize config"}}"#).into())
                        }
                    }
                })
                // Build a message responder
                .and_then(move |socket| {
                    // Get sink and stream
                    let (sink, stream) = socket.split();
                    stream
                        // For all messages until the connection closes
                        .take_while(move |message| Ok(!message.is_close()))
                        // Handle the input and generate output
                        .filter_map(move |message| {
                            // Log the message
                            debug!("Message from Client {}: {:?}", session_id, message);
                            // Handle the message by type
                            GameServer::handle_message(message, session_id, &world)
                        })
                        .forward(sink)
                        .and_then(move |(_, sink)| {

                            // Delete the swarm from this session
                            match w.write() {
                                Ok(mut write_lock) => {
                                    // Get a mutable reference to the world
                                    let world_ref = write_lock.deref_mut();
                                    // Remove the player
                                    world_ref.remove_player(session_id);
                                    // Write lock goes out of scope, world is again available to be read
                                },
                                Err(error) => {
                                    error!("Error getting write lock: {}. Player not removed", error);
                                
                                }
                            };
                            // Send the close message
                            sink.send(OwnedMessage::Close(None))
                        })
                });

            spawn_future(message_handler, "Client Status", &handle);
            
            Ok(())
        });
    info!("Starting the server at {}:{}", hostname, port);
    core.run(server_future).expect("Failed to start server");
}

// TODO: learn what this does and how it works
fn spawn_future<F, I, E>(f: F, desc: &'static str, handle: &Handle)
where
    F: Future<Item = I, Error = E> + 'static,
    E: Debug,
{
    handle.spawn(
        f.map_err(move |e| info!("{}: '{:?}'", desc, e))
            .map(move |_| info!("{}: Finished.", desc)),
    );
}
