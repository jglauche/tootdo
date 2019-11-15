use crate::todolist::*;
use crate::text_nav::*;
use std::error::Error;
use termion::{*,color};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ColorConf {
		header_bg: (u8, u8, u8),
		header_fg: (u8, u8, u8),
		text_bg: (u8, u8, u8),
		text_fg: (u8, u8, u8),
}

impl std::default::Default for ColorConf {
	fn default() -> Self {
		Self {
			header_bg: 	(143, 63, 113),
			header_fg: 	(255, 255, 255),
			text_bg: 		(104, 104, 104),
			text_fg: 		(178, 178, 178),
		}
	}
}

impl ColorConf{
	fn header_bg(&self) -> color::Bg<impl color::Color> {
		let c = &self.header_bg;
		color::Bg(color::Rgb(c.0, c.1, c.2))
	}

	fn header_fg(&self) -> color::Fg<impl color::Color>{
		let c = &self.header_fg;
		color::Fg(color::Rgb(c.0, c.1, c.2))
	}

	fn header_colors(&self) {
		print!("{}{}", self.header_bg(), self.header_fg())
	}

	fn text_bg(&self) -> color::Bg<impl color::Color> {
		let c = &self.text_bg;
		color::Bg(color::Rgb(c.0, c.1, c.2))
	}

	fn text_fg(&self) -> color::Fg<impl color::Color>{
		let c = &self.text_fg;
		color::Fg(color::Rgb(c.0, c.1, c.2))
	}

	fn text_colors(&self) {
		print!("{}{}", self.text_bg(), self.text_fg())
	}


}

fn print_header(s: &str, size: usize) -> Result <(), Box<dyn Error>> {
	let cfg:ColorConf = confy::load("tootdo")?;

	let mut spaces = String::with_capacity(size - s.chars().count());
	for _ in 0..spaces.capacity(){
		spaces.push(' ');
	}
	cfg.header_colors();
	print!("{}{}", s, spaces);
	cfg.text_colors();
	Ok(())
}


pub fn main() -> Result <(), Box<dyn Error>> {

	let size:(u16, u16) = match terminal_size(){
		Ok(res) => { res },
		Err(err) => { panic!("Error: #{:?}", err) },
	};

	let mut list = TodoList::new();
	// dummy data
	let t = String::from("Brush your teeth");
	list.add(t.into());
	let t = String::from("Eat a warm meal");
	list.add(t.into());

	loop{
		print!("{}{}", termion::clear::All, cursor::Goto(1,1) );

		let bg = color::Bg(color::Black);
		print_header("Todo List", size.0.into())?;
		println!("{}{}", cursor::Goto(1,1), bg);

		list.print();

		println!("{}{}", cursor::Goto(1,size.1-1), bg);

		let res = query("New item?", "").expect("not to fail");
		if res != "" {
			list.add(res.into());
		}

	}

	Ok(())
}


