use mysql_eloquent_builder::builder::select_query::{
	SelectQuery,
	EColumn,
	TNewQuery,
	TColumn,
	TCompileQuery,
};

#[test]
fn test_select_query_single_result() {
	let query = SelectQuery::new()
        .table("users".to_string())
    	.add_column(EColumn::Aggregated("total".to_string(), "COUNT(id)".to_string()))
		.compile();

	assert_eq!(query, "SELECT COUNT(id) AS `total` FROM `users`");
}
