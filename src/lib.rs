//! # rcp
//! Lazy Tool to send one File over the network to another machine.
//!
//!
//! ## Description
//! rcp is an alternative way to send one file over the Network to a machine.
//!
//! ## Usage
//!
//! |Mode |Usage |
//! |- |- |
//! |Sender | Give as Argument the target IP and the lokal File to send it to the target |
//! |Listener | Wait for a Sender to Connect to and Save the file in the current Directory |
//!
//!

pub mod listener;
pub mod sender;
