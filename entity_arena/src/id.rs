use id_macro::id;
use serde::{Deserialize, Serialize};

#[id]
pub struct EntityId;

#[id]
pub struct EntityTypeId;

// #[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct Id(Uuid);

// impl std::fmt::Debug for Id {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         self.0.to_string()[0..8].fmt(f)
//     }
// }

// impl Id {
//     #[must_use]
//     pub fn new() -> Self {
//         Id(Uuid::new_v4())
//     }

//     #[must_use]
//     pub fn parse_str(s: &str) -> Self {
//         Id(Uuid::parse_str(s).unwrap())
//     }
// }

// impl Default for Id {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct EntityId(Id);

// impl EntityId {
//     #[must_use]
//     pub fn new() -> Self {
//         EntityId(Id::new())
//     }

//     #[must_use]
//     pub fn parse_str(s: &str) -> Self {
//         EntityId(Id::parse_str(s))
//     }
// }

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
// pub struct EntityTypeId(Id);

// impl EntityTypeId {
//     #[must_use]
//     pub fn parse_str(s: &str) -> Self {
//         EntityTypeId(Id::parse_str(s))
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::Id;

//     #[test]
//     fn can_parse() {
//         let guid_to_parse = "9f19a122-b52f-43b7-b5f4-632d2defb828";

//         let _parsed = Id::parse_str(guid_to_parse);

//         // implicit assert: no panic
//     }
// }
