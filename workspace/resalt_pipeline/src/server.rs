use super::{Broadcaster, BroadcasterError, Client};
use actix_web::web::Data;
use resalt_models::Minion;
use serde_json::{json, Value};
use std::sync::Mutex;

#[derive(Clone)]
pub struct PipelineServer {
    broadcaster: Data<Mutex<Broadcaster>>,
}

impl Default for PipelineServer {
    fn default() -> Self {
        PipelineServer::new()
    }
}

impl PipelineServer {
    pub fn new() -> Self {
        PipelineServer {
            broadcaster: Broadcaster::create(),
        }
    }

    // call broadcaster.handle_client
    pub fn new_client(&self, user_id: String) -> Client {
        self.broadcaster.lock().unwrap().new_client(user_id)
    }

    pub fn update_minion(&self, minion: Minion) {
        self.send(
            "update_minion",
            json!({
                "minion": minion,
            }),
        );
    }

    fn send(&self, name: &str, value: Value) {
        let packet = json!({
            "type": name,
            "content": value,
        });
        self.broadcaster.lock().unwrap().send(&packet.to_string());
    }

    #[allow(dead_code)]
    fn send_to(&self, user_id: &str, name: &str, value: Value) -> Result<(), BroadcasterError> {
        let packet = json!({
            "type": name,
            "content": value,
        });
        self.broadcaster
            .lock()
            .unwrap()
            .send_to(user_id, &packet.to_string())
    }
}
