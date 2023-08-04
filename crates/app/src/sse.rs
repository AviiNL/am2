#![allow(unused)]

use convert_case::{Case, Casing};
use futures::StreamExt;
use gloo_net::{eventsource::futures::EventSource, http::Request};
use leptos::{create_rw_signal, spawn_local, RwSignal, SignalUpdate};
use serde::{de::DeserializeOwned, Serialize};
use serde_type_name::type_name;
use std::{rc::Rc, sync::RwLock};

#[allow(unused)]
#[derive(Clone)]
pub struct Sse {
    #[cfg(feature = "hydrate")]
    event_source: Rc<RwLock<EventSource>>,
    signals: Rc<RwLock<anymap::AnyMap>>,
}

impl Default for Sse {
    fn default() -> Self {
        Self {
            #[cfg(feature = "hydrate")]
            event_source: Rc::new(RwLock::new(
                EventSource::new("/sse/v2/subscribe").expect("to be able to create event source"),
            )),
            signals: Rc::new(RwLock::new(anymap::AnyMap::new())),
        }
    }
}

impl Sse {
    pub fn subscribe<T>(&self) -> RwSignal<T>
    where
        T: Default + DeserializeOwned + Serialize,
    {
        if let Some(signal) = self.signals.read().unwrap().get::<RwSignal<T>>() {
            return *signal;
        }

        let default = T::default();

        let channel = type_name(&default).unwrap().to_case(Case::Snake);
        let c = channel.clone();

        let signal = create_rw_signal(default);
        self.signals.write().unwrap().insert(signal);

        #[cfg(feature = "hydrate")]
        spawn_local(async move {
            let response = Request::get(&format!("/api/v2/sse/{}", c)).send().await.unwrap();
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
