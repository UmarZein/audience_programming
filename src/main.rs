use crate::simulator::*;
use handler::electoral_handler;
use host::run_host;
use std::{sync::mpsc, time::Duration};
use utils::*;

mod handler;
mod host;
mod simulator;
mod trie;
mod utils;

static UPDATE_INTERVAL: Duration = Duration::from_millis(5000);

// NOTE: "most common prefix" means "longest most frequent prefix"

// TL;DR:
// - Node::most_common_prefix => function to calculate most common prefix
// - electoral_handler => what to do with most common prefix
// - UPDATE_INTERVAL => how often to update most common prefix (5 seconds on prime's first vim-with-me
// session with chat)
// - host => twitch chat listener, updates every UPDATE_INTERVAL

fn main() {
    // host channel
    let (tx, rx) = mpsc::channel();

    // spawn 20 twitch chat users
    let mut users: Vec<_> = (0..20)
        .map(|uid| simulate_twitch_chat(tx.clone(), simple_hash(uid)))
        .collect();
    drop(tx); // do not let it linger

    // start host who will take the chat stream
    let host = run_host(rx, electoral_handler());
    while let Some(chatter_thread) = users.pop() {
        chatter_thread.join().unwrap();
    }
    host.join().unwrap();
}
