//########################################## EMITTING FOLLOWING MODULES ##########################################//

mod api_display;
pub use api_display::*;

pub mod tui;

//############################################### EXTERNAL IMPORTS ###############################################//

use minimad::{OwningTemplateExpander, TextTemplate};
use termimad::crossterm::style::Color;
use termimad::*;

//############################################### INTERNAL IMPORTS ###############################################//

mod errors;
pub use errors::*;

//################################################ MEMBER IMPORTS ################################################//

use reachapi::{RawOuts, ArxivOutput};
use metadata::VERSION;

//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

pub trait TerminalDisplay {
    fn display_in_terminal(raw_outs: Vec<RawOuts>) -> Result<(), ReachTuiError>;
    fn get_display_template() -> &'static str;
    fn get_expander() -> OwningTemplateExpander<'static> {
        let mut expander = OwningTemplateExpander::new();
        expander
            .set("app-name", "Reach")
            .set("app-version", VERSION);

        expander
    }
    fn make_skin() -> MadSkin {
        let mut skin = MadSkin::default();
        skin.set_headers_fg(Color::AnsiValue(178));
        skin.headers[2].set_fg(gray(22));
        skin.bold.set_fg(Color::Yellow);
        skin.italic.set_fg(Color::White);
        skin.scrollbar.thumb.set_fg(Color::AnsiValue(178));
        skin.table_border_chars = ROUNDED_TABLE_BORDER_CHARS;
        skin.paragraph.align = Alignment::Left;
        skin.table.align = Alignment::Left;
        skin
    }
}