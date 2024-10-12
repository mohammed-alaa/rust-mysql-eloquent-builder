pub fn compile_group_by(group_by: Vec<String>) -> String {
	match group_by.is_empty() {
		true => "".to_string(),
		false => format!("GROUP BY `{}`", group_by.join("`, `")),
	}
}

pub fn compile_sort(sorts: Vec<String>) -> String {
	match sorts.is_empty() {
		true => "".to_string(),
		false => format!("ORDER BY {}", sorts.join(", ")),
	}
}

pub fn compile_columns(columns: Vec<String>) -> String {
	match columns.is_empty() {
		true => "*".to_string(),
		false => columns.join(", "),
	}
}
