pub use crate::builder::{
	traits::column::TColumn,
	enums::columns::EColumn,
	base::Query,
	verbs::select::structs::Select,
};

impl TColumn for Query<Select> {
	/**
	 * Add column to query
	 * @param column
	 */
	fn add_column(mut self, column: EColumn) -> Self {
		let column = match column {
			EColumn::Aggregated(alias, aggregate) => format!("{} AS `{}`", aggregate, alias),
			EColumn::Column(name) => format!("`{}`", name),
		};
		self.base.columns.push(column);
		self
	}

	/**
	 * Set columns to query
	 * @param columns Vec<EColumn>
	 */
	fn add_columns(mut self, columns: Vec<EColumn>) -> Self {
		for column in columns {
			self = self.add_column(column);
		};
		self
	}
}
