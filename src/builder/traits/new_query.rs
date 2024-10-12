#[allow(unused)]
pub trait TNewQuery {
	fn new() -> Self;
	fn table(self, table: String) -> Self;
}
