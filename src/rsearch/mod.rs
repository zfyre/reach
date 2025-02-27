mod knowledge_graph;
mod utils;

// External Imports
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;

// Crate Imports
use crate::{
    display::RawOuts,
    apis::gemini_query,
    errors::ReachError,
    config::{ApiConfig, ApiKeys},
};
use utils::get_markdown;

#[derive(Parser, Debug)]
pub struct Rsearch {}