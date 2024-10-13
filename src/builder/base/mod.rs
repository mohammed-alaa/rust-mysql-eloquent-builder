pub use crate::builder::traits::new_query::TNewQuery;

pub struct Query<T: TNewQuery> {
	pub base: T,
	pub table: String,
}

impl<T: TNewQuery> Query<T> {
	pub fn new() -> Self {
		Query {
			base: T::new(),
			table: String::new(),
		}
	}

	pub fn table(mut self, table: String) -> Self {
		self.table = table;
		self
	}
}
