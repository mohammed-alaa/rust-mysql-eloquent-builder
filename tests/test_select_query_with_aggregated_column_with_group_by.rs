use mysql_eloquent_builder::builder::enums::columns::EColumn;
use mysql_eloquent_builder::builder::verbs::select::impls::new_query::Query;
use mysql_eloquent_builder::builder::traits::column::TColumn;
use mysql_eloquent_builder::builder::traits::group_by::TGroup;
use mysql_eloquent_builder::builder::traits::query_compile::TCompileQuery;

#[test]
fn test_select_query_with_aggregated_column_with_group_by() {
	let query = Query::select()
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
