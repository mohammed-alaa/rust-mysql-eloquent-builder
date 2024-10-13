mod structs;

pub use crate::builder::{
	base::Query,
	traits::new_query::TNewQuery,
};
use self::structs::Transaction;

impl TNewQuery for Transaction {
	fn new() -> Self {
		Transaction {}
	}
}

impl Query<Transaction> {
	pub fn transaction() -> Self {
		Self::new()
	}

	pub fn start(self) -> String {
		"START TRANSACTION".to_string()
	}

	pub fn begin(self) -> String {
		"BEGIN".to_string()
	}

	pub fn commit(self) -> String {
		"COMMIT".to_string()
	}

	pub fn rollback(self) -> String {
		"ROLLBACK".to_string()
	}
}
