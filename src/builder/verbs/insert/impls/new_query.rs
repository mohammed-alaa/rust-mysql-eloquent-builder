pub use crate::builder::{
	base::Query,
	traits::new_query::TNewQuery,
	verbs::insert::structs::Insert,
};

impl TNewQuery for Insert {
	fn new() -> Self {
		Insert {
			columns: Vec::new(),
		}
	}
}

impl Query<Insert> {
	pub fn insert() -> Self {
		Self::new()
	}
}
