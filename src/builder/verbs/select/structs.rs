pub struct Select {
	pub(super) columns: Vec<String>,
	pub(super) group_by: Vec<String>,
	pub(super) sorts: Vec<String>,
}
