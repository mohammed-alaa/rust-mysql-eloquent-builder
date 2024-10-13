use super::compilers::*;

pub use crate::builder::{
	traits::query_compile::TCompileQuery,
	base::Query,
	verbs::select::structs::Select,
};

impl TCompileQuery for Query<Select> {
	fn compile(self) -> String {
		let mut sql = "SELECT".to_string();
		sql = format!("{} {}", sql, compile_columns(self.base.columns));
		sql = format!("{} FROM `{}`", sql, self.table);

		if !self.base.sorts.is_empty() {
			sql = format!("{} {}", sql, compile_sort(self.base.sorts));
		}

		if !self.base.group_by.is_empty() {
			sql = format!("{} {}", sql, compile_group_by(self.base.group_by));
		}

		sql.trim_end().to_string()
	}

	fn to_sql(self) -> String {
		self.compile()
	}
}
