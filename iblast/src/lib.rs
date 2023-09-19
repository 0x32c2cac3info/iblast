#![allow(unused_imports, dead_code)]
use anyhow::{Error, Result};

/// Actions are plain data members that contain data only relevant to their responsibilities.
/// Like a form of country-specific currency, they may originate from different places and only
/// be accepted at specific places. Ideally, they should be immutable.
pub mod action {
    use crate::model::Detail;
    
    pub struct Create<'a> {
        pub detail: &'a Detail,
    }

    impl<'a> TryFrom<&'a Detail> for Create<'a> {
        type Error = anyhow::Error;
        
        fn try_from(detail: &'a Detail) -> anyhow::Result<Self, Self::Error> {
            Ok(Self { detail })
        }
    }
}

/// A dispatcher is a single object that broadcasts actions/events to all registered stores.
/// Stores need to register for events when the application starts.
/// When a dispatcher receives a change action from a dispatcher, it will _dispatch_ that action
/// to all registered stores.
pub mod dispatcher {
    use std::sync::mpsc::{Sender, Receiver};
    use std::collections::BTreeMap;
    
    pub fn broadcast<'action>(topics: &'action [&'action str]) -> Broadcast {
        Broadcast::new(topics)
    }

    pub struct Broadcast<'a, 'b> where 'b: 'a {
        submission: BTreeMap<String, DisHandle<'a, 'b>>,
    }

    impl<'a, 'b> Broadcast<'a, 'b> {
        pub fn new(topics: &[&'b str]) -> Self {
            let mut it = BTreeMap::<String, DisHandle<'a, 'b>>::new();
            let submission = (move || {
                for s in topics {
                    let handle = DisHandle::new(s).unwrap();
                    let topic_name = s.to_string();
                    let _ = &mut it.insert(topic_name, handle);
                }
                it
            })();
            Broadcast { submission }
        }
    }

    pub struct DisHandle<'a, 'b> where 'b: 'a {
        name: &'b str,
        sender: Sender<&'a [u8]>,
    }
    
    impl<'a, 'b> DisHandle<'a, 'b> where 'b: 'a {
        pub fn new(topic_name: &'b str) -> anyhow::Result<Self> {
            let sender = crate::store::register_dispatcher(topic_name)?;
            Ok(Self { name: topic_name, sender })
        }
    }
}

/// Stores manages the state. It can store both domain state and user interface state.
/// Store and state are different concepts. State is the data value. Store is a behavior
/// object that manages state through methods.
///
/// Stores manages multiple types. It is the single source of truth for data (having those types).
/// Requesting state changes is done by passing an action to the dispatcher.
/// Stores listen for all actions and decide on which ones to act upon. Once a store has submitted
/// a change action to a dispatcher, it will then emit a change event.
///
/// A Store is an event emitter; they don't nest or take other stores as dependencies.
pub mod store {
    use anyhow::bail;
    use std::sync::Mutex;
    use std::sync::mpsc::{Sender, Receiver};
    use std::collections::HashSet;
    use once_cell::sync::OnceCell;
    
    pub fn register_dispatcher<'a, 'b>(topic_name: &'a str) -> anyhow::Result<Sender<&'b [u8]>> {
        static TAKEN: OnceCell<Mutex<HashSet<String>>> = OnceCell::new();
        let taken = TAKEN.get_or_init(|| { 
            let /* mut */ t = HashSet::<String>::new();
            Mutex::new(t)
        });
        let mut taken = taken.lock().unwrap();
        let key = String::from(topic_name);        
        if taken.contains(topic_name) {
            bail!("Duplicate keys forbidden");    
        }
        let (sender, _receiver) = std::sync::mpsc::channel::<&[u8]>();
        let _ = &mut taken.insert(key);
        Ok(sender)
    }
}

/// Action creators are functions that create and dispatch behavior.
pub mod creators {
    use anyhow::Result;
    use crate::model::Detail;
    use crate::action::Create;
    
    pub fn add_detail(detail: &Detail) -> Result<Create> {
        let create_detail = Create::try_from(detail);
        create_detail
    }
}

/// View is the user interface component. It is responsible for rendering
/// the user interface and for handling the user interaction.
///
/// Typical reponsibilities of views include:
/// - listening for store changes and re-rendering
/// - maintaining a local distinction of presentation and container views
///   where presentation views don't connect to dispatchers or stores, only
///   communicating via their own properties; container views are connected to
///   stores and provide the data for presentation components.
///
/// Container views, being connected to stores and dispatchers, listen for events
/// from stores and provide the data for presentation components.
/// They get new data using the store's public getter methods and then pass that data
/// down the views tree.
///
/// Oh yeah, views maintain a tree structure by the way.
///
pub mod view {
    
}

/// Utilities that action creators use
/// to engage in high-level or encapsulated behaviors requiring
/// multiple API call functions.
///
/// It may be appropriate to move a given function out of the API and into the Utilities
/// when it causes the call stack of a given scope in an API function to call an API
/// at a depth greater than 2.
pub mod util {}

/// API call invocations should happen in action creators.
/// Complex or compound API calls should be consolidated into the util module
/// so that the API module is mostly comprised of near-indivisible operations.
///
/// When called to update the user interface, the web API call will be met with:
/// - maybe a validation util
/// - a call to update the associated store
/// - when the store is updated it will emit a change event and as a result the view that
///   listens for that event will re-render
pub mod api {

}

pub mod model {
    #[derive(Debug, Clone, Copy)]
    pub struct Detail {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_detail() {
        use crate::model::Detail;
        let detail = Detail {};
        let create: Result<action::Create> = creators::add_detail(&detail);
        assert!(create.is_ok());
    }

    #[test]
    fn dispatch_create_detail() {
        use crate::dispatcher as dp;
        use crate::action::Create;
        use crate::model::Detail;

        let to = vec!["Detail"];
        let tx: dp::Broadcast = dp::broadcast(&to[..]);

        todo!("Try broadcasting a create action")
    }

    #[test]
    fn dispatcher_topic_exclusivity() {
        // no duplicates
        let _original_handle = store::register_dispatcher("pikachu");
        let duplicate_handle = store::register_dispatcher("pikachu");
        assert!(duplicate_handle.is_err());
    }
}