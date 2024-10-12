#[allow(unused)]
pub trait TGroup {
	fn group_by(self, column: String) -> Self;
	fn group_by_columns(self, columns: Vec<String>) -> Self;
}
