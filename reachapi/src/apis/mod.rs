// Emitting Following Modules
mod arxiv;
pub use arxiv::*;

mod gemini;
pub use gemini::*;

mod google;
pub use google::*;


// External Imports
use std::collections::HashMap;
use serde_json::{Value, json};
use reqwest::Client;

// Internal Imports
use crate::config::{ArxivConfig, ArxivKeys};
use crate::ReachApiError;

// Member Imports


// Common Functions (May be Emitted)

#[derive(Debug)]
pub enum RawOuts {
    RawGeminiOut(String),
    RawArxivOut(ArxivOutput),
    RawGoogleOut((String, String)),
}