
pub mod cli{
	use crate::todolist::*;
	use crate::text_nav::*;
	use std::error::Error;

	pub fn main() -> Result <(), Box<dyn Error>> {
		let mut list = todolist::TodoList::new();
		let t = String::from("Brush your teeth");
		list.add(t.into());
		let t = String::from("Eat a warm meal");
		list.add(t.into());
		list.print();

		let res = text_nav::query("New item?", "").expect("not to fail");
		list.add(res.into());
		list.print();

		Ok(())
	}


}
