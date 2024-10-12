#[allow(unused)]
pub trait TCompileQuery {
	fn compile(self) -> String;
	fn to_sql(self) -> String;
}
