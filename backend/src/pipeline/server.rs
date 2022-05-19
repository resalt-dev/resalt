use super::{Broadcaster, Client};
use crate::prelude::*;
use actix_web::web::Data;
use serde_json::{json, Value};
use std::sync::Mutex;

#[derive(Clone)]
pub struct PipelineServer {
    broadcaster: Data<Mutex<Broadcaster>>,
}

impl PipelineServer {
    pub fn new() -> Self {
        PipelineServer {
            broadcaster: Broadcaster::create(),
        }
    }

    // call broadcaster.handle_client
    pub fn new_client(&self) -> Client {
        self.broadcaster.lock().unwrap().new_client()
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
}
