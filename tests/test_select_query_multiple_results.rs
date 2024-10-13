use mysql_eloquent_builder::builder::select_query::{
	SelectQuery,
	EColumn,
	TNewQuery,
	TColumn,
	TCompileQuery
};

#[test]
fn test_select_query_multiple_results() {
	let query = SelectQuery::new()
		.table("users".to_string())
		.add_column(
			EColumn::Column("id".to_string())
		)
		.add_columns(vec![
			EColumn::Column("name".to_string()),
			EColumn::Column("email".to_string()),
		])
		.compile();

	assert_eq!(query, "SELECT `id`, `name`, `email` FROM `users`");
}
