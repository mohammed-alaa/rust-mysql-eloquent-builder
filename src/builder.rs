#[allow(unused)]
pub enum QueryType {
	Select,
	Insert,
	Update,
	Delete,
}

#[allow(unused)]
pub trait Group {
	fn group_by(self, column: String) -> Self;
	fn group_by_columns(self, columns: Vec<String>) -> Self;
}

#[allow(unused)]
pub trait CompileQuery {
	fn compile(self) -> String;
}
