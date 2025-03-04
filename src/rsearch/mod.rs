mod knowledge_graph;
mod utils;

// External Imports
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use serde_json::Value;

// Crate Imports
use crate::{
    display::RawOuts,
    apis::{gemini_query, google_search},
    errors::ReachError,
    config::{ApiConfig, ApiKeys},
};

#[derive(Parser, Debug)]
pub struct Rsearch {}