
pub mod text_nav {
	use std::io::Write;
	use std::error::Error;
	use console::{Key, Term};

	pub fn query(query: &str, default: &str) -> Result<(String), Box<dyn Error>> {
		let mut buf = String::from(default);
		let mut term = Term::stdout();
		let mut clear = true;

		loop{
			if clear == true {
				term.clear_line()?;
				term.write(&query.to_string().as_bytes())?;
				term.write(b" ")?;
				term.write(&buf.to_string().as_bytes())?;
				clear = false;
			}
			match term.read_key()? {
				Key::Char(c) => {
					buf = buf + &c.to_string();
					term.write(&c.to_string().as_bytes())?;
				},
				Key::Backspace => {
					if buf.len() > 0 {
						buf.truncate(buf.len()-1);
						clear = true;
					}
				},
				Key::Enter => {
					term.write_line("")?;
					break
				},
				_ => {}
			}
		}
		Ok(buf)
	}
}
