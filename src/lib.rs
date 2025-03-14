//########################################## EMITTING FOLLOWING MODULES ##########################################//

mod errors;
pub use errors::ReachError;

//############################################### EXTERNAL IMPORTS ###############################################//

use std::{fmt, io};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

//############################################### INTERNAL IMPORTS ###############################################//


//################################################ MEMBER IMPORTS ################################################//

use reachdb::ReachdbError;
use rsearch::RsearchError;
use reachapi::ReachApiError;
use reachtui::ReachTuiError;
pub use metadata::*;

//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

