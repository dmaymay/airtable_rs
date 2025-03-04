/// Re-export the client struct from `client.rs` so it’s available at `client::AirtableClient`.
pub mod client;
pub mod error; // we'll define this in the next commit

pub use client::AirtableClient;