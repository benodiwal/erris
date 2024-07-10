use crate::error::Result;
use console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

pub fn prompt(text: &str) -> Result<String> {
	let theme = ColorfulTheme {
		prompt_style: Style::new().for_stderr().color256(45),
		prompt_prefix: style(">> ".to_string()).color256(45).for_stderr(),
		..ColorfulTheme::default()
	};

	let input = Input::with_theme(&theme);
	let res = input.with_prompt(text).interact_text()?;

	Ok(res)
}

pub enum Cmd {
	Quit,
	Chat(String),
}

impl Cmd {
	pub fn from_input(input: impl Into<String>) -> Self {
		let input = input.into();
		
		if input == r"\q" {
			Self::Quit
		} else {
			Self::Chat(input)
		}

	}
}
