pub use crate::builder::{
	base::Query,
	traits::new_query::TNewQuery,
	verbs::select::structs::Select,
};

impl TNewQuery for Select {
	fn new() -> Self {
		Select {
			columns: Vec::new(),
			group_by: Vec::new(),
			sorts: Vec::new(),
		}
	}
}

impl Query<Select> {
	pub fn select() -> Self {
		Self::new()
	}
}
