mod gemini;
mod google;
mod arxiv;

pub use gemini::*;
pub use google::*;
pub use arxiv::*;

// External imports
use reqwest::Client;
use serde_json::{Value, json};
use std::collections::HashMap;

// Crate imports
use crate::{
    config::{ArxivConfig, ArxivKeys},
    display::RawOuts,
    errors::ReachError
};

