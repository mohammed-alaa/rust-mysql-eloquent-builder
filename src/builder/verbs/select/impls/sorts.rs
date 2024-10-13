pub use crate::builder::{
	traits::sort::TSort,
	enums::sort_directions::ESortDirection,
	base::Query,
	verbs::select::structs::Select,
};

impl TSort for Query<Select> {
	/**
	 * Sort query
	 * @param column String
	 * @param direction ESortDirection
	 * @return Self
	 */
	fn sort(mut self, column: String, direction: ESortDirection) -> Self {
		let direction = match direction {
			ESortDirection::ASC => "ASC",
			ESortDirection::DESC => "DESC",
		};

		self.base.sorts.push(format!("`{}` {}", column, direction));
		self
	}
}
