use crate::trie::Node;
use crate::UPDATE_INTERVAL;
use colored::Colorize;
use std::sync::mpsc;
use std::thread::{self, sleep, JoinHandle};
use std::time::{Duration, Instant};

/// `rx`: receiver of entries (e.g.: stream forwarded from twitch chat)
/// `electoral_handler`: sender-part of the handler which will send the most
/// common prefix of received rx strings over the interval `UPDATE_INTERVAL` or
/// double of it to its receiver
pub fn run_host(
    rx: mpsc::Receiver<(usize, String)>,
    electoral_handler: mpsc::Sender<String>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        println!(
            "{}{}",
            "[host]: ".yellow().bold(),
            "spawned host thread".yellow().italic(),
        );
        let mut mark = Instant::now();
        let mut trie = Node::new();
        loop {
            match rx.recv_timeout(UPDATE_INTERVAL) {
                Ok((_, entry)) => {
                    let dur_since = mark.elapsed();
                    if dur_since > UPDATE_INTERVAL {
                        let (s, _) = trie.most_common_prefix();
                        electoral_handler
                            .send(s.clone())
                            .expect("unable to send message to electoral handler");
                        mark = Instant::now();
                        trie = Node::new();
                        println!(
                            "{}{}",
                            "[host]: ".yellow().bold(),
                            format!("current most common prefix is {s:?}")
                                .yellow()
                                .italic(),
                        );
                    }
                    trie.insert(&entry);
                }
                Err(e) => {
                    println!(
                        "{}{}",
                        "[host]: ".yellow().bold(),
                        format!(
                            "did not receive any messages for {} seconds",
                            UPDATE_INTERVAL.as_secs()
                        )
                        .yellow()
                        .italic(),
                    );
                    let (s, _) = trie.most_common_prefix();
                    electoral_handler
                        .send(s.clone())
                        .expect("unable to send message to electoral handler");
                    println!(
                        "{}{}",
                        "[host]: ".yellow().bold(),
                        format!("current most common prefix is {s:?}")
                            .yellow()
                            .italic(),
                    );
                    if e == mpsc::RecvTimeoutError::Disconnected {
                        println!(
                            "{}{}",
                            "[host]: ".yellow().bold(),
                            "all clients have disconnected. please restart the program...".yellow()
                        );
                        sleep(Duration::from_millis(300));
                        break;
                    }
                    mark = Instant::now();
                    trie = Node::new();
                }
            }
        }
    })
}
