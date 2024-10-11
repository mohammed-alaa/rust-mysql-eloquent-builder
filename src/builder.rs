#[allow(unused)]
pub enum EQueryType {
	Select,
	Insert,
	Update,
	Delete,
}

#[allow(unused)]
pub enum EColumn {
	Aggregated(String, String),
	Column(String),
}

#[allow(unused)]
pub enum ESortDirection{
	ASC,
	DESC,
}

#[allow(unused)]
pub trait TCompileQuery {
	fn compile(self) -> String;
	fn to_sql(self) -> String;
}

#[allow(unused)]
pub trait TGroup {
	fn group_by(self, column: String) -> Self;
	fn group_by_columns(self, columns: Vec<String>) -> Self;
}

#[allow(unused)]
pub trait TNewQuery {
	fn new() -> Self;
	fn table(self, table: String) -> Self;
}

#[allow(unused)]
pub trait TColumn {
	fn add_column(self, column: EColumn) -> Self;
	fn add_columns(self, column: Vec<EColumn>) -> Self;
}

#[allow(unused)]
pub trait TSort {
	fn sort(self, column: String, direction: ESortDirection) -> Self;
}
