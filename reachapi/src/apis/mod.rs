//########################################## EMITTING FOLLOWING MODULES ##########################################//

mod arxiv;
pub use arxiv::*;
mod gemini;
pub use gemini::*;
mod google;
pub use google::*;

//############################################### EXTERNAL IMPORTS ###############################################//

use std::collections::HashMap;
use serde_json::{Value, json};
use reqwest::Client;

//############################################### INTERNAL IMPORTS ###############################################//

use crate::config::{ArxivConfig, ArxivKeys};
use crate::ReachApiError;
use crate::ReachConfig;
use crate::ReachConfigKeys;

//################################################ MEMBER IMPORTS ################################################//


//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

#[derive(Debug)]
pub enum RawOuts {
    RawGeminiOut(String),
    RawArxivOut(ArxivOutput),
    RawGoogleOut((String, String)),
}