use mysql_eloquent_builder::builder::enums::columns::EColumn;
use mysql_eloquent_builder::builder::verbs::select::impls::new_query::Query;
use mysql_eloquent_builder::builder::traits::column::TColumn;
use mysql_eloquent_builder::builder::traits::query_compile::TCompileQuery;

#[test]
fn test_select_query_single_result() {
	let query = Query::select()
        .table("users".to_string())
    	.add_column(EColumn::Aggregated("total".to_string(), "COUNT(id)".to_string()))
		.compile();

	assert_eq!(query, "SELECT COUNT(id) AS `total` FROM `users`");
}
