mod select_query;
mod builder;

fn main() {
}

#[cfg(test)]
mod tests {
	use super::{
		builder::{EColumn, TNewQuery, TColumn, TGroup, TCompileQuery},
		select_query::SelectQuery,
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

		assert_eq!(query, "SELECT id, name, email FROM users");
	}

	#[test]
	fn test_select_query_single_result() {
		let query = SelectQuery::new()
	        .table("users".to_string())
	    	.add_column(EColumn::Aggregated("total".to_string(), "COUNT(id)".to_string()))
			.compile();

		assert_eq!(query, "SELECT COUNT(id) AS `total` FROM users");
	}

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

		assert_eq!(query, "SELECT user_id, SUM(profit) AS `total_profit` FROM profit GROUP BY user_id");
	}
}
