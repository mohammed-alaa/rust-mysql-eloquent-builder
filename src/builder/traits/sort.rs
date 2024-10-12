use crate::builder::enums::sort_directions::ESortDirection;

#[allow(unused)]
pub trait TSort {
	fn sort(self, column: String, direction: ESortDirection) -> Self;
}
