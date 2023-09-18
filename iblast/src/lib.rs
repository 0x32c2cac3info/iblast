pub(crate) mod common {
    use std::{default, u128};

    use anyhow::{Error, Result};
    use chrono::NaiveDate;
    use chronoutil::RelativeDuration;
    pub(crate) struct License;

    impl License {
        fn new() -> Self {
            Self {}
        }
        fn renew_for<Time: Into<RelativeDuration> + Sized>(
            &mut self,
            time: Time,
        ) -> Result<(), Error> {
            todo!("Add renewal logic");
            Ok(())
        }
        fn renew_until<Time: Into<NaiveDate> + Sized>(&mut self, time: Time) -> Result<(), Error> {
            todo!("Add renew-until logic");
            Ok(())
        }
        fn check_expired<Time: Into<NaiveDate> + Sized>(&self, time: Time) -> bool {
            todo!("Add check expiration db logic");
            false
        }
        fn cancel(mut self) -> Result<Self, Error> {
            todo!("Call drop record logic");
            Ok(self)
        }
    }

    use std::rc::{Rc, Weak};

    pub struct Account {
        pub id: ID,
        pub is_guest: bool,
        pub is_sub: bool,
        pub is_premium: bool,
        pub user: Rc<User>,
    }

    pub struct ID(pub u128);

    pub struct User {
        pub id: ID,
        pub addresses: Option<Vec<Address>>,
        pub first_name: Option<String>,
        pub middle_name: Option<String>,
        pub last_name: Option<String>,
        pub phones: Option<Vec<Phone>>,
        pub cards: Option<Vec<Card>>,
        account: Weak<Account>,
    }

    pub enum Field {
        Required(Option<String>),
        Optional(Option<String>),
        Empty,
    }

    use std::ptr::NonNull;
    pub struct RequiredField(pub NonNull<String>);
    pub struct Address {
        line1: Field,
        line2: Field,
    }

    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum Phone {
        Mobile(u16),
        Home(u16),
        Work(u16),
    }

    impl Default for Phone {
        fn default() -> Self {
            Self::Mobile(<_>::default())
        }
    }

    pub struct Card {
        brand: CardBrand,
        number: Option<String>,
    }

    #[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
    pub enum CardBrand {
        #[default]
        Nil,
        Visa,
        MasterCard,
        Amex,
        Discover,
        Jcb2,
        Other,
    }
}

/// Actions are plain data members that contain data only relevant to their responsibilities.
/// Like a form of country-specific currency, they may originate from different places and only
/// be accepted at specific places. Ideally, they should be immutable.
pub(crate) mod action {}

/// A dispatcher is a single object that broadcasts actions/events to all registered stores.
/// Stores need to register for events when the application starts.
/// When a dispatcher receives a change action from a dispatcher, it will _dispatch_ that action
/// to all registered stores.
pub(crate) mod dispatcher {}

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
pub(crate) mod store {}

/// Action creators are functions that create and dispatch behavior.
pub(crate) mod creators {
}

pub(crate) mod view {}

/// Utilities that action creators use
/// to engage in high-level or encapsulated behaviors requiring 
/// multiple API call functions.
/// 
/// It may be appropriate to move a given function out of the API and into the Utilities
/// when it causes the call stack of a given scope in an API function to call an API 
/// at a depth greater than 2. 
pub(crate) mod util {}

/// API call invocations should happen in action creators.
/// Complex or compound API calls should be consolidated into the util module
/// so that the API module is mostly comprised of near-indivisible operations.
/// 
/// When called to update the user interface, the web API call will be met with: 
/// - maybe a validation util
/// - a call to update the associated store
/// - when the store is updated it will emit a change event and as a result the view that
///   listens for that event will re-render
pub(crate) mod api {}