#![no_std]
//! Provides access to the Iota Client API

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod builder;
#[macro_use]
pub mod client;
pub mod api;
pub mod error;
pub mod extended;
// #[cfg(feature = "quorum")]
// pub mod quorum;
pub mod response;
mod util;

pub use builder::ClientBuilder;
pub use client::Client;
pub use error::*;
pub use response::*;
pub use util::{bytes_to_trytes, str_to_trytes};

pub use anyhow::{Result, Error};
pub use spin::RwLock;


#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

#[cfg(feature = "std")]
#[macro_use]
extern crate std;

#[cfg(not(feature = "std"))]
pub use core::convert::TryInto;

#[cfg(not(feature = "std"))]
pub use alloc::{
    boxed::{
        self,
        Box,
    },
    fmt,
    rc::{
        self,
        Rc,
    },
    string::{
        self,
        String,
        ToString,
    },
    sync::{
        self,
        Arc,
    },
    vec::{
        self,
        Vec,
    },
    borrow::{
        self,
        ToOwned
    },
};

#[cfg(feature = "std")]
pub use std::{
    boxed::{
        self,
        Box,
    },
    rc::{
        self,
        Rc,
    },
    fmt,
    string::{
        self,
        String,
        ToString,
    },
    sync::{
        self,
        Arc,
    },
    vec::{
        self,
        Vec,
    },
    borrow::{
        self,
        ToOwned,
    },
    convert::TryInto
};

pub use hashbrown::{
    hash_map,
    hash_set,
    HashMap,
    HashSet,
};
