use mysql_eloquent_builder::builder::verbs::select::impls::new_query::Query;

#[test]
fn test_transaction_start() {
	let transaction = Query::transaction();
	let query = transaction.start();
	assert_eq!(query, "START TRANSACTION");
}

#[test]
fn test_transaction_begin() {
	let transaction = Query::transaction();
	let query = transaction.begin();
	assert_eq!(query, "BEGIN");
}

#[test]
fn test_transaction_commit() {
	let transaction = Query::transaction();
	let query = transaction.commit();
	assert_eq!(query, "COMMIT");
}

#[test]
fn test_transaction_rollback() {
	let transaction = Query::transaction();
	let query = transaction.rollback();

	assert_eq!(query, "ROLLBACK");
}
