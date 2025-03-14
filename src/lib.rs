//########################################## EMITTING FOLLOWING MODULES ##########################################//

mod errors;
pub use errors::ReachError;

//############################################### EXTERNAL IMPORTS ###############################################//

use tokio;
use clap::Parser;
use std::{str::FromStr, fmt, io};
use std::collections::HashMap;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

//############################################### INTERNAL IMPORTS ###############################################//


//################################################ MEMBER IMPORTS ################################################//

use reachdb::ReachdbError;
use rsearch::{RsearchError, build_kg_iteratively};
use metadata::*;

//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

