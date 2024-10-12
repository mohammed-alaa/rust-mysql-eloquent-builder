mod compilers;

use crate::builder::{TGroup, TNewQuery, TColumn, EColumn, TSort, ESortDirection, TCompileQuery};
use compilers::{
	compile_columns, compile_sort, compile_group_by
};

pub struct SelectQuery {
	table: Option<String>,
	columns: Vec<String>,
	group_by: Vec<String>,
	sorts: Vec<String>,
	// where_conditions: Vec<String>,
}

impl TNewQuery for SelectQuery {
	fn new() -> Self {
		Self {
			table: None,
			columns: vec![],
			group_by: vec![],
			sorts: vec![],
			// where_conditions: vec![],
		}
	}

	fn table(mut self, table: String) -> Self {
		self.table = Some(table);
		self
	}
}

impl TCompileQuery for SelectQuery {
	fn compile(self) -> String {
		let mut sql = "SELECT".to_string();
		sql = format!("{} {}", sql, compile_columns(self.columns));
		sql = format!("{} FROM `{}`", sql, self.table.unwrap());

		if !self.sorts.is_empty() {
			sql = format!("{} {}", sql, compile_sort(self.sorts));
		}

		if !self.group_by.is_empty() {
			sql = format!("{} {}", sql, compile_group_by(self.group_by));
		}

		sql.trim_end().to_string()
	}

	fn to_sql(self) -> String {
		self.compile()
	}
}

impl TColumn for SelectQuery {
	/**
	 * Add column to query
	 * @param column
	 */
	fn add_column(mut self, column: EColumn) -> Self {
		let column = match column {
			EColumn::Aggregated(alias, aggregate) => format!("{} AS `{}`", aggregate, alias),
			EColumn::Column(name) => format!("`{}`", name),
		};
		self.columns.push(column);
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

impl TGroup for SelectQuery {
	fn group_by(mut self, column: String) -> Self {
		self.group_by.push(column);
		self
	}

	fn group_by_columns(mut self, columns: Vec<String>) -> Self {
		self.group_by.extend(columns);
		self
	}
}

impl TSort for SelectQuery {
	fn sort(mut self, column: String, direction: ESortDirection) -> Self {
		let direction = match direction {
			ESortDirection::ASC => "ASC",
			ESortDirection::DESC => "DESC",
		};

		self.sorts.push(format!("`{}` {}", column, direction));
		self
	}
}
