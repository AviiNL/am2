#![allow(unused)]

use futures::StreamExt;
use gloo_net::{eventsource::futures::EventSource, http::Request};
use leptos::{create_rw_signal, spawn_local, RwSignal, SignalUpdate};
use serde::de::DeserializeOwned;
use std::{rc::Rc, sync::RwLock};

pub const DEFAULT_SSE_URL: &str = "/sse/v2";

#[allow(unused)]
#[derive(Clone)]
pub struct Sse {
    #[cfg(feature = "hydrate")]
    event_source: Rc<RwLock<EventSource>>,
    signals: Rc<RwLock<anymap::AnyMap>>, // _ should be "any" type or whatever implements DeserializeOwned, but not bound to the struct.
}

impl Default for Sse {
    fn default() -> Self {
        let url = format!("{}/subscribe", DEFAULT_SSE_URL);

        Self {
            #[cfg(feature = "hydrate")]
            event_source: Rc::new(RwLock::new(
                EventSource::new(&url).expect("to be able to create event source"),
            )),
            signals: Rc::new(RwLock::new(anymap::AnyMap::new())),
        }
    }
}

impl Sse {
    pub fn subscribe<T>(&self, channel: &'static str) -> RwSignal<T>
    where
        T: Default + DeserializeOwned,
    {
        if let Some(signal) = self.signals.read().unwrap().get::<RwSignal<T>>() {
            return *signal;
        }

        let signal = create_rw_signal(T::default());
        self.signals.write().unwrap().insert(signal);

        #[cfg(feature = "hydrate")]
        spawn_local(async move {
            let response = Request::get(&format!("/api/v2/sse/{channel}")).send().await.unwrap();
            let data: T = response.json().await.unwrap();
            signal.update(|s| *s = data);
        });

        #[cfg(feature = "hydrate")]
        let mut subscription = self.event_source.write().unwrap().subscribe(channel).unwrap();

        #[cfg(feature = "hydrate")]
        spawn_local(async move {
            while let Some(Ok((_, msg))) = subscription.next().await {
                let data = msg.data().as_string().unwrap();
                let t: T = serde_json::from_str::<T>(&data).unwrap();
                signal.update(|s| *s = t);
            }
        });

        signal
    }
}
