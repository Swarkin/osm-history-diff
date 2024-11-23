use std::fmt::Formatter;

pub enum ElementType {
	Node,
	Way,
	Relation,
}

impl std::fmt::Display for ElementType {
	fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
		match self {
			ElementType::Node => f.write_str("node"),
			ElementType::Way => f.write_str("way"),
			ElementType::Relation => f.write_str("relation"),
		}
	}
}


pub type ElementHistory = Vec<HistoryEntry>;

pub struct HistoryEntry {
	pub r#type: ElementType,
	//pub id: u32,
	//pub timestamp: String,
	pub version: u32,
	//pub changeset: u32,
	//pub user: String,
	//pub nodes: Vec<osm_primitives::ReferencedNode>,
	pub tags: Vec<osm_primitives::Tag>,
}
