#![feature(async_await, await_macro)]
#![allow(unused_mut)]

//! Lusion Web Application.

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate assert_matches;

macro_rules! box_async {
    {$($t:tt)*} => {
        ::futures::future::FutureExt::boxed(async move { $($t)* })
    };
}

pub mod endpoints;
pub mod error;
pub mod middleware;
pub mod request;
pub mod response;
pub mod security;

#[cfg(test)]
mod test_helpers;
