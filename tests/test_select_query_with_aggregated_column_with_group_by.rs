use mysql_eloquent_builder::builder::select_query::{
	SelectQuery,
	EColumn,
	TNewQuery,
	TColumn,
	TCompileQuery,
	TGroup,
};

#[test]
fn test_select_query_with_aggregated_column_with_group_by() {
	let query = SelectQuery::new()
		.table("profit".to_string())
		.add_column(
			EColumn::Column("user_id".to_string())
		)
		.add_column(
			EColumn::Aggregated("total_profit".to_string(), "SUM(profit)".to_string())
		)
		.group_by("user_id".to_string())
		.compile();

	assert_eq!(query, "SELECT `user_id`, SUM(profit) AS `total_profit` FROM `profit` GROUP BY `user_id`");
}
