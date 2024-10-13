pub use crate::builder::{
	traits::group_by::TGroup,
	base::Query,
	verbs::select::structs::Select,
};

impl TGroup for Query<Select> {
	fn group_by(mut self, column: String) -> Self {
		self.base.group_by.push(column);
		self
	}

	fn group_by_columns(mut self, columns: Vec<String>) -> Self {
		self.base.group_by.extend(columns);
		self
	}
}
