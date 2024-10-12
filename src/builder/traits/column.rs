use crate::builder::enums::columns::EColumn;

#[allow(unused)]
pub trait TColumn {
	fn add_column(self, column: EColumn) -> Self;
	fn add_columns(self, column: Vec<EColumn>) -> Self;
}
