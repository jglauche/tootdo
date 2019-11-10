pub mod cli{
	use crate::todolist::*;
	use crate::text_nav::*;
	use std::error::Error;
	use termion::{*,color};


	fn print_header(s: &str, size: usize) {
		let bg = color::Bg(color::Rgb(143, 63, 113));
		let mut spaces = String::with_capacity(size - s.chars().count());
		for _ in 0..spaces.capacity(){
			spaces.push(' ');
		}

		print!("{}{}{}", bg, s, spaces);
	}


	pub fn main() -> Result <(), Box<dyn Error>> {

		let size:(u16, u16) = match terminal_size(){
			Ok(res) => { res },
			Err(err) => { panic!("Error: #{:?}", err) },
		};

		let mut list = todolist::TodoList::new();
		// dummy data
		let t = String::from("Brush your teeth");
		list.add(t.into());
		let t = String::from("Eat a warm meal");
		list.add(t.into());

		loop{
			print!("{}{}", termion::clear::All, cursor::Goto(1,1) );

			let bg = color::Bg(color::Black);
			print_header("Todo List", size.0.into());
			println!("{}{}", cursor::Goto(1,1), bg);

			list.print();

			println!("{}{}", cursor::Goto(1,size.1-1), bg);

			let res = text_nav::query("New item?", "").expect("not to fail");
			if res != "" {
				list.add(res.into());
			}

		}

		Ok(())
	}


}
