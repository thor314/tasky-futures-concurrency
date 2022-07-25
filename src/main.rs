//! A batteries-included binary template.
//! https://docs.rs/tasky/latest/tasky/
//! https://blog.yoshuawuyts.com/postfix-spawn/
//! https://docs.rs/futures-concurrency/latest/futures_concurrency/

// TODO: remove these when ready
// #![feature(into_future)] // stable in 1.64
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::{future::IntoFuture, time::Duration};

use anyhow::Result;
use clap::Parser;
// use futures::{
//   channel::mpsc,
//   executor::{self, ThreadPool},
//   future::join,
//   Future, StreamExt,
// };
use tokio::time::sleep;
use utils::MyError;
use validator::{Validate, ValidationError};

#[cfg(test)] mod tests;
mod utils;
#[tokio::main]
async fn main() -> Result<()> {
  let context = utils::setup()?;
  Ok(())
}

// usual spawning, eager:
fn usual_spawn() -> impl futures::Future { tokio::spawn(sleep(Duration::from_secs(2))) }

// tasky future
async fn tasky() {
  use std::future;

  use futures_concurrency::prelude::*;
  use tasky::prelude::*; // re-exports FutureExt, containing spawn and spawn_local

  // builder builds a future to evaluate lazily, doesn't call until awaited
  let builder = sleep(Duration::from_secs(2)).spawn().name("sleep1".into());
  let builder_2 = sleep(Duration::from_secs(2)).spawn();

  let builder_3 = sleep(Duration::from_secs(2)).spawn().into_future();
  // can optionally declare an async block here, unnecessary:
  let builder_4 = sleep(Duration::from_secs(2)).spawn().into_future();

  let handles = cute::c![sleep(Duration::from_secs(n)).spawn().into_future(), for n in 1..4];

  // then we use the futures_concurrency crate for a method to join the Builders, since how else
  // would we join this custom task-builder?

  // this was annoying, needed to into_future() my builders
  // let out = (builder, builder_2).join().await;
  let out = (builder_3, builder_4).join().await;
  let handles_out = handles.join().await;

  // example use of futures concurrency, can use tuple, array, vec, whatever
  let a = future::ready(1u8);
  let b = future::ready("hello");
  let c = future::ready(3u16);
  assert_eq!((a, b, c).join().await, (1, "hello", 3));

  todo!()
}
