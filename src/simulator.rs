use crate::utils::simple_hash;
use crate::UPDATE_INTERVAL;
use colored::Colorize;
use std::sync::mpsc;
use std::thread::{self, sleep, JoinHandle};

pub fn simulate_twitch_chat(tx: mpsc::Sender<(usize, String)>, uid: usize) -> JoinHandle<()> {
    thread::spawn(move || {
        let c = (simple_hash(uid) as f64) / 1000.0;
        sleep(UPDATE_INTERVAL.mul_f64(c));
        for part in [PART1, PART2, PART3, PART4] {
            let message = String::from(part[uid % part.len()]);
            tx.send((uid, message.clone()))
                .expect("error when sending message");
            println!(
                "{}{}",
                format!("[twitch chat#user-{:03}]: ", uid).purple().bold(),
                format!("{:?}", message).truecolor(255, 179, 255).italic()
            );
            sleep(UPDATE_INTERVAL);
        }
    })
}

static PART1: &[&str] = &[
    "const sum = arr",
    "const sum = arr.re",
    "const tot = arr.reduce",
    "const x = 0",
    "const sum = ",
    "const total",
    "let sum = 0; arr.forEach(n => sum += n);",
    "let tot =",
    "let total",
    "let sum = 0; for(let i = 0",
    "const sum = arr.reduce((n, {Amount})",
    "const x = arr.reduce",
];
// should be "const sum = "

static PART2: &[&str] = &[
    "arr.reduce((a, b)",
    "arr.reduce((x,",
    "arr.reduce((i,",
    "arr.reduce",
    "arr.reduce",
    "arr.map(item =>",
    "arr.map(item",
];
// followed by "arr.reduce"

static PART3: &[&str] = &[
    "((a, b) => a + b, 0);",
    "((x, y) => x + y, 0);",
    "((x, y) => x + y, 0);",
    "((a, b)",
    "((x, y)",
    "((sum, new))",
];
// then "((x, y)"

static PART4: &[&str] = &[" => x + y, 0);", "=> x + y, 0);"];
// finally " => x + y, 0);"
