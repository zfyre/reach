//########################################## EMITTING FOLLOWING MODULES ##########################################//

mod knowledge_graph;
mod utils;
pub use utils::*;
mod errors;
pub use errors::RsearchError;

//############################################### EXTERNAL IMPORTS ###############################################//

use clap::Parser;
use regex::Regex;
use std::{collections::HashMap, fmt};
use serde_json::Value;
use log::{info, trace};
use tokio;

//############################################### INTERNAL IMPORTS ###############################################//

//################################################ MEMBER IMPORTS ################################################//

use reachapi::{
    ApiConfig, ApiKeys, RawOuts,
    ReachApiError, google_search, gemini_query
};
use reachdb::{Reachdb, ReachdbError, UserDefinedRelationType};

//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

#[derive(Parser, Debug)]
pub struct Rsearch {}
pub use knowledge_graph::build_kg_iteratively;