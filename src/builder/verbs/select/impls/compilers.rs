/**
 * Compile a group by query
 * @param group_by - group by clause
 * @return String - compiled group by clause
 */
pub fn compile_group_by(group_by: Vec<String>) -> String {
	match group_by.is_empty() {
		true => "".to_string(),
		false => format!("GROUP BY `{}`", group_by.join("`, `")),
	}
}

/**
 * Compile sorts to a string
 * @param sorts - sorts to compile
 * @return String - compiled sorts clause
 */
pub fn compile_sort(sorts: Vec<String>) -> String {
	match sorts.is_empty() {
		true => "".to_string(),
		false => format!("ORDER BY {}", sorts.join(", ")),
	}
}

/**
 * Compile columns to a string
 * @param columns - columns to compile
 * @return String - compiled columns clause
 */
pub fn compile_columns(columns: Vec<String>) -> String {
	match columns.is_empty() {
		true => "*".to_string(),
		false => columns.join(", "),
	}
}
