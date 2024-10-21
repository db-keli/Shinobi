// src/lib.rs
pub mod decoder;
pub mod errors;
pub mod worker;

#[cfg(test)]
mod tests;

pub mod api;
pub mod cli;
pub mod service;
