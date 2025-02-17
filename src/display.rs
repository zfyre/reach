use super::VERSION;
use crate::apis::{ArxivOutput, ReachError};
use colored::Colorize;
use {
    minimad::{OwningTemplateExpander, TextTemplate},
    termimad::crossterm::style::Color::*,
    termimad::*,
};

pub enum RawOuts<'a> {
    RawGeminiOut(&'a str),
    RawArxivOut(ArxivOutput),
    RawGoogleOut((String, String)),
}
pub trait TerminalDisplay {
    fn display_in_terminal(raw_outs: Vec<RawOuts>) -> Result<(), ReachError>;
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
        skin.set_headers_fg(AnsiValue(178));
        skin.headers[2].set_fg(gray(22));
        skin.bold.set_fg(Yellow);
        skin.italic.set_fg(Magenta);
        skin.scrollbar.thumb.set_fg(AnsiValue(178));
        skin.table_border_chars = ROUNDED_TABLE_BORDER_CHARS;
        skin.paragraph.align = Alignment::Left;
        skin.table.align = Alignment::Left;
        skin
    }
}

pub struct GoogleTerminalDisplay;
pub struct GeminiTerminalDisplay;
pub struct ArxivTerminalDisplay;

impl TerminalDisplay for GoogleTerminalDisplay {
    fn get_display_template() -> &'static str {
        r#"
        -----------
        # ${app-name} v${app-version}
        ## Google Search Mode

        |:-:|:-:|
        |**Title**|**URL**|
        |:-|:-|
        ${module-rows
        |**${module-name}**|${module-key}|
        }
        |-|-|
        "#
    }
    fn display_in_terminal(raw_outs: Vec<RawOuts>) -> Result<(), ReachError> {
        let mut expander = Self::get_expander();
        for raws in raw_outs {
            match raws {
                RawOuts::RawGoogleOut((title, link)) => {
                    expander
                        .sub("module-rows")
                        .set("module-name", title)
                        .set("module-key", link);
                }
                _ => (),
            }
        }
        // use the data to build the markdown text and print it
        let skin = Self::make_skin();
        let template = TextTemplate::from(Self::get_display_template());
        let text = expander.expand(&template);
        let (width, _) = terminal_size();
        let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
        print!("{}", fmt_text);
        Ok(())
    }
}

impl TerminalDisplay for ArxivTerminalDisplay {
    fn get_display_template() -> &'static str {
        r#"
        -----------
        # ${app-name} v${app-version}
        ## Arxiv Search Mode
        ${module-rows
        **${module-name}** 
        (*${module-key}*):
        ${module-description}
        }

        "#
    }
    fn display_in_terminal(raw_outs: Vec<RawOuts>) -> Result<(), ReachError> {
        let mut expander = Self::get_expander();
        for raws in raw_outs {
            match raws {
                RawOuts::RawArxivOut(ArxivOutput {
                    title,
                    url,
                    summary,
                }) => {
                    println!("{}", summary);
                    expander
                        .sub("module-rows")
                        .set("module-name", title)
                        .set("module-key", url)
                        .set("module-description", format!("{}...", &summary[..500.min(summary.len())]));
                }
                _ => (),
            }
        }
        // use the data to build the markdown text and print it
        let skin = Self::make_skin();
        let template = TextTemplate::from(Self::get_display_template());
        let text = expander.expand(&template);
        let (width, _) = terminal_size();
        let fmt_text = FmtText::from_text(&skin, text, Some(2*width as usize));
        print!("{}", fmt_text);
        Ok(())
    }
}

/// File to configure the display the output in the terminal

pub fn gemini_display_output(markdown_text: &str) {
    println!("{}", markdown_text)
}

pub fn arxiv_display_output(arxiv_output: &Vec<ArxivOutput>) {
    for out in arxiv_output {
        println!("{}\n{}\n\n", out.url.yellow(), out.summary);
    }
}

#[cfg(test)]
mod tests {
    use colored::Colorize;
    use termimad::{crossterm::style::Color::*, MadSkin, *};

    const TEST_STRING: &str = "\"Grokking\" is a term coined by Robert A. Heinlein in his 1961 science fiction novel *Stranger in a Strange Land*. It means to understand something so thoroughly and completely that you become one with it. It goes beyond intellectual understanding and involves intuition, empathy, and deep connection.\n\nHere's a breakdown of what \"grokking\" entails:\n\n*   **Deep, intuitive understanding:** It's not just knowing facts and figures, but having an instinctp of how something works, its purpose, and its implications.\n*   **Empathy and connection:** It involves understanding something from the inside out, being able to put yourself in its place and see the world from its perspective.\n*   **Incorporation into your being:** When you grok something, it becomes a part of you. It changes the way you think and interact with the world.\n*   **Agreement and acceptance:** It often implies a deep level of acceptance and agreement with the thing you are grokking. It's not just understanding it, but also aligning with it.\n*   **A sense of oneness:** Ultimately, grokking is about achieving a sense of oneness with the subject of your understanding.\n\n**In essence, grokking is about achieving a holistic and profound understanding that transcends mere intellectual knowledge.**\n\n**Examples:**\n\n*   A programmer might grok a programming language when they understand not just the syntax and commands, but also the underlying philosophy and design principles, allowing them to write elegant and efficient code.\n*   A musician might grok a piece of music when they understand not only the notes and rhythms, but also the emotions and intentions behind it, allowing them to perform it with passion and authenticity.\n*   A therapist might grok a patient when they understand not only the patient's words and actions, but also their underlying motivations, fears, and desires, allowing them to provide effective treatment.\n\nWhile a fictional word, \"grokking\" has become a useful term to describe a superior level of understanding in various fields.\n```python\n def main():\n\treturn \"Hello World\"\n```";

    fn make_skin() -> MadSkin {
        let mut skin = MadSkin::default();
        skin.set_headers_fg(AnsiValue(178));
        skin.headers[2].set_fg(gray(22));
        skin.bold.set_fg(Yellow);
        skin.italic.set_fg(Magenta);
        skin.scrollbar.thumb.set_fg(AnsiValue(178));
        skin.table_border_chars = ROUNDED_TABLE_BORDER_CHARS;
        skin
    }
    #[test]
    fn test_bold() {
        let s = "**To be Bold**, this is not bold, **This is bold again!**";
        let result = s
            .replace("**", "")
            .split("**")
            .enumerate()
            .map(|(i, text)| {
                if i % 2 == 1 {
                    format!("{}", text.bold())
                } else {
                    text.to_string()
                }
            })
            .collect::<String>();
        println!("{}", result);
    }

    #[test]
    fn test_newline() {
        let test_result = TEST_STRING.replace("\\n", "\r\n");
        // println!("{test_result}");

        let skin = make_skin();
        skin.print_text(&test_result);
    }
}
