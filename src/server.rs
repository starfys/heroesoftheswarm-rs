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

use futures::{Future, Sink, Stream};
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
use std::thread;
use std::time::Duration;
use tokio_core::reactor::{Core, Handle};
use websocket::message::{Message, OwnedMessage};
use websocket::server::InvalidConnection;
use websocket::async::Server;
use world::World;

/// Represents a server for the game
// TODO: populate this with parameters
pub struct GameServer {
    /// Hostname for the websocket to listen on
    hostname: String,
    /// Port for the websocket to listen on
    port: u16,
    /// Number of server updates per second
    update_freq: u64,
    /// The game's world
    world: Arc<RwLock<World>>,
    /// A counter used to assign player IDs to session
    id_counter: AtomicUsize,
}
impl GameServer {
    /// Constructor
    /// hostname: address for the websocket to listen on
    /// port: port for the websocket to listen on
    /// width: width of the game world
    /// height: height of the game world
    /// update_freq: server updates per second
    pub fn new(hostname: &str, port: u16, width: f32, height: f32, update_freq: u64) -> Self {
        GameServer {
            hostname: hostname.into(),
            port: port,
            update_freq: update_freq,
            world: Arc::new(RwLock::new(World::new(width, height))),
            id_counter: AtomicUsize::new(0),
        }
    }
    /// Starts the server
    pub fn start() {}
    /// Handles an incoming websocket message
    fn handle_message(message: OwnedMessage, world: &Arc<RwLock<World>>) -> Option<OwnedMessage> {
        match message {
            // Handle incoming text data
            OwnedMessage::Text(data) => match data.as_ref() {
                // This is sent if the client wants the game's state
                "U" => {
                    // Get a readable reference to the world
                    // (locks until the world is not being written)
                    let world = world.read().unwrap();
                    Some(OwnedMessage::Binary(vec![0; 20000]))
                }
                _ => None,
            },
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
    let update_freq: u64 = 1;
    // Create the world
    let world: Arc<RwLock<World>> = Arc::new(RwLock::new(World::new(1000.0, 1000.0)));
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
            info!(
                "Last update took {}s, {}ns",
                last_update_time.as_secs(),
                last_update_time.subsec_nanos()
            );
            // Sleep for some amount of time
            thread::sleep(update_delta - last_update_time);
            // Lock the world for writing
            let mut write_lock = world.write().unwrap();
            // Get a mutable reference to the world
            let world_ref = write_lock.deref_mut();
            // Update the world
            last_update_time = world_ref.update()
            // Write lock goes out of scope, world is again available to be read
        }
    });
    // Used to assign IDs to connections (players)
    let id_counter: AtomicUsize = AtomicUsize::new(0);
    // Used for serving
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    // Bind to an address
    let server = Server::bind(format!("{}:{}", hostname, port), &handle).unwrap();
    // This future represents what this server is going to do.
    // Handles a stream of incoming connections
    let server_future = server.incoming()
        // Handle errors
        .map_err(move |InvalidConnection { error, .. }| error)
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
            // Get an ID for this connection
            let session_id: usize = id_counter.fetch_add(1, AtomicOrdering::SeqCst);
            info!("New session id: {}", session_id);
            // accept the request to be a ws connection if it does
            let message_handler = upgrade
                // Use our protocol
                .use_protocol("heroesoftheswarm")
                // Accept the message
                .accept()
                // Respond so the client knows the connection succeeded 
                .and_then(move |(socket, _)| socket.send(Message::binary(vec![1,3,3,7]).into()))
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
                            info!("Message from Client: {:?}", message);
                            // Handle the message by type
                            GameServer::handle_message(message, &world)
                        })
                        .forward(sink)
                        .and_then(move |(_, sink)| {
                            sink.send(OwnedMessage::Close(None))
                        })
                });

            spawn_future(message_handler, "Client Status", &handle);
            Ok(())
        });
    info!("Starting the server at {}:{}", hostname, port);
    core.run(server_future).unwrap();
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
