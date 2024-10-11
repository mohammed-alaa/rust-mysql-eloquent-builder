use crate::builder::{CompileQuery, Group};

#[allow(unused)]
pub enum EColumn {
	Aggregated(String, String),
	Column(String),
}

#[allow(unused)]
pub struct SelectQuery {
	table: Option<String>,
	columns: Vec<String>,
	group_by: Vec<String>,
}

#[allow(unused)]
pub trait NewQuery {
	fn new() -> Self;
	fn table(self, table: String) -> Self;
}

#[allow(unused)]
pub trait TColumn {
	fn add_column(self, column: EColumn) -> Self;
	fn add_columns(self, column: Vec<EColumn>) -> Self;
}

impl NewQuery for SelectQuery {
	fn new() -> Self {
		Self {
			table: None,
			columns: vec![],
			group_by: vec![],
		}
	}

	fn table(mut self, table: String) -> Self {
		self.table = Some(table);
		self
	}
}

impl CompileQuery for SelectQuery {
	fn compile(self) -> String {
		let mut sql = "SELECT".to_string();
		sql = format!("{} {}", sql, compile_columns(self.columns));
		sql = format!("{} FROM `{}`", sql, self.table.unwrap());
		sql = format!("{} {}", sql, compile_group_by(self.group_by));
		sql
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

impl Group for SelectQuery {
	fn group_by(mut self, column: String) -> Self {
		self.group_by.push(column);
		self
	}

	fn group_by_columns(mut self, columns: Vec<String>) -> Self {
		self.group_by.extend(columns);
		self
	}
}

fn compile_columns(columns: Vec<String>) -> String {
	match columns.is_empty() {
		true => format!(" *"),
		false => columns.join(", "),
	}
}

fn compile_group_by(group_by: Vec<String>) -> String {
	if group_by.is_empty() {
		return "".to_string();
	}

	format!("GROUP BY `{}`", group_by.join("`, `"))
}
