// use termion::style;
use colored::Colorize;

use crate::apis::ArxivOutput;

/// File to configure the display the output in the terminal

pub fn gemini_display_output(markdown_text: &str) {
    println!("{markdown_text}")
}

pub fn arxiv_display_output(arxiv_output: Vec<ArxivOutput>) {
    for out in arxiv_output {
        println!("{}\n{}\n\n", out.url.yellow(), out.summary);
    }
}

#[cfg(test)]
mod tests {

    const TEST_STRING: &str = "\"Grokking\" is a term coined by Robert A. Heinlein in his 1961 science fiction novel *Stranger in a Strange Land*. It means to understand something so thoroughly and completely that you become one with it. It goes beyond intellectual understanding and involves intuition, empathy, and deep connection.\n\nHere's a breakdown of what \"grokking\" entails:\n\n*   **Deep, intuitive understanding:** It's not just knowing facts and figures, but having an instinctp of how something works, its purpose, and its implications.\n*   **Empathy and connection:** It involves understanding something from the inside out, being able to put yourself in its place and see the world from its perspective.\n*   **Incorporation into your being:** When you grok something, it becomes a part of you. It changes the way you think and interact with the world.\n*   **Agreement and acceptance:** It often implies a deep level of acceptance and agreement with the thing you are grokking. It's not just understanding it, but also aligning with it.\n*   **A sense of oneness:** Ultimately, grokking is about achieving a sense of oneness with the subject of your understanding.\n\n**In essence, grokking is about achieving a holistic and profound understanding that transcends mere intellectual knowledge.**\n\n**Examples:**\n\n*   A programmer might grok a programming language when they understand not just the syntax and commands, but also the underlying philosophy and design principles, allowing them to write elegant and efficient code.\n*   A musician might grok a piece of music when they understand not only the notes and rhythms, but also the emotions and intentions behind it, allowing them to perform it with passion and authenticity.\n*   A therapist might grok a patient when they understand not only the patient's words and actions, but also their underlying motivations, fears, and desires, allowing them to provide effective treatment.\n\nWhile a fictional word, \"grokking\" has become a useful term to describe a superior level of understanding in various fields.\n";


    #[test]
    fn test_bold() {
        // let s = "**To be Bold**, this is not bold, **This is bold again!**";
        // let result = s.replace("**", "")
        //     .split("**")
        //     .enumerate()
        //     .map(|(i, text)| {
        //         if i % 2 == 1 {
        //             format!("{}{}{}",
        //                 style::Bold,
        //                 text,
        //                 style::Reset)
        //         } else {
        //             text.to_string()
        //         }
        //     })
        //     .collect::<String>();
        // println!("{}", result);
    }

    #[test]
    fn test_italics() {

    }

    #[test]
    fn test_heading() {

    }

    #[test]
    fn test_underline() {

    }

    #[test]
    fn test_multiline_code() {

    }

    #[test]
    fn test_inline_code() {

    }

    #[test]
    fn test_newline() {
        let test_result = TEST_STRING.replace("\\n", "\r\n");
        println!("{test_result}");
        
    }
    

}