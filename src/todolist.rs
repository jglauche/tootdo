use ansi_term::Style;

use chrono::prelude::*;

pub struct TodoItem{
	pub name: String,
	pub completed: bool,
	pub repeat_days: u32,
	pub self_care: bool,
	pub private: bool,
	pub date: DateTime<Utc>,
}
// probably also want some way to define in which order to repeat things, for example brush teeth mornings should be first, brush teeth evenings last

impl Default for TodoItem {
	fn default() -> TodoItem {
		TodoItem {
			name: String::from(""),
			completed: false,
			repeat_days: 0,
			self_care: false,
			private: false,
			date: Utc::now(),
		}
	}
}

impl PartialEq for TodoItem {
	fn eq(&self, other: &Self) -> bool {
		// FIXME
		self.name == other.name && self.completed == other.completed
	}
}

impl TodoItem{
	pub fn new(name: String) -> TodoItem{
		TodoItem{name: name,
		..Default::default()
		}
	}

	pub fn local_date(&self) -> chrono::Date<Local>{
		DateTime::<Local>::from(self.date).date()
	}

	pub fn print(&self) -> (){
		if self.completed {
			println!("  {}", Style::new().strikethrough().paint(&self.name));
		} else {
			println!("  {}", self.name);
		}
	}
}

impl From<String> for TodoItem{
	fn from(item: String) -> Self{
		TodoItem::new(item)
	}
}

pub fn create(name: String) -> TodoItem{
	TodoItem::new(name)
}

pub struct TodoList{
	pub list: Vec<TodoItem>,
}

impl TodoList{
	pub fn new() -> TodoList{
		TodoList{
			list: Vec::new()
		}
	}

	pub fn add(&mut self, item: TodoItem) -> (){
		self.list.push(item);
	}

	pub fn remove(&mut self, idx: usize) -> (){
		self.list.remove(idx);
	}

	pub fn print(&self) -> (){
		for i in 0..self.list.len() {
			self.list[i].print();
		}
	}
}



