use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use convert_case::{Case, Casing};
use error::Result;

use axum::response::sse::Event;
use serde::Serialize;
use serde_type_name::type_name;
use tokio::sync::broadcast::{channel, Receiver, Sender};

#[derive(Debug, Clone)]
pub struct SseService {
    tx: Sender<Event>,
    latest: Arc<RwLock<HashMap<String, String>>>,
}

impl Default for SseService {
    fn default() -> Self {
        Self {
            tx: channel(100).0,
            latest: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl SseService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subscribe(&self) -> Receiver<Event> {
        self.tx.subscribe()
    }

    pub fn get_latest(&self, name: &str) -> Option<String> {
        let latest = self.latest.read().unwrap();
        latest.get(name).cloned()
    }

    #[allow(unused)]
    pub fn push<T>(&self, data: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let name = type_name(&data)?.to_case(Case::Snake);
        let json = serde_json::to_string(data)?;

        if let Some(latest) = self.get_latest(&name) {
            if latest == json {
                return Ok(());
            }
        }

        let mut latest = self.latest.write().unwrap();
        latest.insert(name.clone(), json.clone());

        let event = Event::default().event(name).data(json);
        self.tx.send(event)?;
        Ok(())
    }
}
