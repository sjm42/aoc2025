// lib.rs

pub use std::{
    cmp,
    io::{self, BufRead},
};

pub use anyhow::{anyhow, bail};
pub use clap::Parser;
pub use tracing::*;

pub mod config;
pub use config::*;

// EOF
