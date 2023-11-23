mod color;

pub use color::{install_color_eyre, Color};
use inquire::{error::InquireResult, Text};

/// Prompt the user for input if the value is None
///
/// Typically used when the user can provide a value via CLI or prompt
pub fn text_prompt_when_none(prompt: &str, val: Option<String>) -> InquireResult<String> {
    Ok(if let Some(val) = val {
        val
    } else {
        Text::new(prompt).prompt()?
    })
}
