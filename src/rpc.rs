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
                message_type: "c".into(),
                message: ResponseMessage::CONFIG(configuration),
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
}

/// Represents configuration
#[derive(Serialize)]
pub struct Configuration {
    /// The player's ID
    player_id: usize,
}

impl Configuration {
    /// Constructor
    pub fn new(player_id: usize) -> Self {
        Configuration {
            player_id: player_id,
        }
    }
}
