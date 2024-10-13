use mysql_eloquent_builder::builder::select_query::{
	SelectQuery,
	EColumn,
	TNewQuery,
	TColumn,
	TCompileQuery,
	TSort,
	ESortDirection
};

#[test]
fn test_select_query_multiple_results_sort_asc() {
	let query = SelectQuery::new()
		.table("users".to_string())
		.add_column(
			EColumn::Column("id".to_string())
		)
		.add_columns(vec![
			EColumn::Column("name".to_string()),
			EColumn::Column("email".to_string()),
		])
		.sort("name".to_string(), ESortDirection::ASC)
		.compile();

	assert_eq!(query, "SELECT `id`, `name`, `email` FROM `users` ORDER BY `name` ASC");
}
