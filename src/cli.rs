/*use std::io;
use std::io::Write;
*/
pub mod cli{
	use crate::todoitem::*;
	use crate::todolist::*;
	use std::error::Error;


	pub fn main() -> Result <(), Box<dyn Error>> {
		let mut list = todolist::TodoList::new();
		let t = todoitem::create(String::from("Brush your teeth"));
		list.add(t);
		let t = todoitem::create(String::from("Eat a warm meal"));
		list.add(t);
		let t = todoitem::create(String::from("put on clothes"));
		list.add(t);
		list.print();

		Ok(())
	}


}
