use super::{Broadcaster, Client};
use actix_web::web::Data;
use std::{collections::HashMap, sync::Mutex};

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
        self.broadcaster.lock().unwrap().new_client(HashMap::new())
    }
}
