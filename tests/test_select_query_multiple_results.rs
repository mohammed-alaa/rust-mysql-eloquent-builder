use mysql_eloquent_builder::builder::enums::columns::EColumn;
use mysql_eloquent_builder::builder::verbs::select::impls::new_query::Query;
use mysql_eloquent_builder::builder::traits::column::TColumn;
use mysql_eloquent_builder::builder::traits::query_compile::TCompileQuery;

#[test]
fn test_select_query_multiple_results() {
	let query = Query::select()
		.table("users".to_string())
		.add_column(EColumn::Column("id".to_string()))
		.add_columns(vec![
			EColumn::Column("name".to_string()),
			EColumn::Column("email".to_string()),
		])
		.compile();

	assert_eq!(query, "SELECT `id`, `name`, `email` FROM `users`");
}
