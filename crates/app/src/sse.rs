use std::{cell::RefCell, collections::HashMap};

use convert_case::{Case, Casing};
use futures::StreamExt;
use gloo_net::{eventsource::futures::EventSource, http::Request};
use leptos::{create_rw_signal, spawn_local, RwSignal};
use serde::{de::DeserializeOwned, Serialize};
use serde_type_name::type_name;

pub const DEFAULT_SSE_URL: &str = "/sse/v2";

#[allow(unused)]
#[derive(Clone)]
pub struct Sse {
    #[cfg(feature = "hydrate")]
    event_source: RefCell<EventSource>,
    signals: HashMap<&'static str, RwSignal<_>>, // _ should be "any" type or whatever implements DeserializeOwned, but not bound to the struct.
}

impl Default for Sse {
    fn default() -> Self {
        let url = format!("{}/subscribe", DEFAULT_SSE_URL);

        Self {
            #[cfg(feature = "hydrate")]
            event_source: RefCell::new(EventSource::new(&url).expect("to be able to create event source")),
        }
    }
}

impl Sse {
    pub async fn subscribe<T>(&mut self, channel: &'static str) -> RwSignal<T>
    where
        T: DeserializeOwned,
    {
        // // if a signal already exists for this channel, return it
        // if let Some(signal) = self.signals.get(channel) {
        //     return signal.clone();
        // }

        // make an api call to /api/v2/sse/{channel} to get the initial data to construct the RwSignal
        let response = Request::get(&format!("/api/v2/sse/{channel}")).send().await.unwrap();
        let data: T = response.json().await.unwrap();

        let signal = create_rw_signal(data);

        #[cfg(feature = "hydrate")]
        let mut subscription = self.event_source.borrow_mut().subscribe(channel).unwrap();

        #[cfg(feature = "hydrate")]
        spawn_local(async move {
            // as the subscription is moved in here, the borrow_mut never ends, and thus we can't re-borrow it for another subscription.
            while let Some(Ok((event_type, msg))) = subscription.next().await {
                let data = msg.data().as_string().unwrap();
                let t: T = serde_json::from_str::<T>(&data).unwrap();
            }
        });

        signal
    }
}
