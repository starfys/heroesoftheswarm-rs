extern crate serde_json;

use world::WorldState;

/// Represents a response sent to the client
#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "mt")]
    /// The type of message
    message_type: String,
    /// The contents of the message
    message: ResponseMessage,
}
impl Response {
    pub fn new(message: ResponseMessage) -> Self {
        match message {
            ResponseMessage::WORLD(world) => Response {
                message_type: "w".into(),
                message: ResponseMessage::WORLD(world),
            },
            ResponseMessage::CONFIG(configuration) => Response {
                message_type: "i".into(),
                message: ResponseMessage::CONFIG(configuration),
            },
            ResponseMessage::COMPILE(compile_result) => Response {
                message_type: "c".into(),
                message: ResponseMessage::COMPILE(compile_result),
            },
        }
    }
    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}
/// Represents the contents of a message
#[derive(Serialize)]
pub enum ResponseMessage {
    /// Sends the world state
    #[serde(rename = "world")]
    WORLD(WorldState),
    /// Sends configuration
    #[serde(rename = "config")]
    CONFIG(Configuration),
    /// Sends a compilation result
    #[serde(rename = "compile")]
    COMPILE(CompileResult),
}

/// Represents configuration
#[derive(Serialize)]
pub struct Configuration {
    /// The player's ID
    player_id: usize,
}

/// Represents the output of a compilation
#[derive(Serialize)]
pub struct CompileResult {
    /// Whether the compilation succeeded
    success: bool,
    /// Error if applicable
    error: String,
}

impl CompileResult {
    /// Constructor
    pub fn new(success: bool, error: String) -> Self {
        CompileResult {
            success: success,
            error: error,
        }
    }
}

impl Configuration {
    /// Constructor
    pub fn new(player_id: usize) -> Self {
        Configuration {
            player_id: player_id,
        }
    }
}

/// A vector in 2d space
/// Used for representing coordinates in
/// the viewport sent for screen updates
#[derive(Deserialize)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

/// A request for compilation
#[derive(Deserialize)]
pub struct CompileRequest {
    pub program: String,
}
