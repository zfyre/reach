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

//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

pub const AUTHOR: &str = "Me <kshitiz4kaushik@gmail.com>";
pub const VERSION: &str = "1.0.0";
pub const CONFIG_FILE: &str = ".reach-config";


