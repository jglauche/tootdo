pub mod todolist{
	use crate::todoitem::todoitem::TodoItem;

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

}
