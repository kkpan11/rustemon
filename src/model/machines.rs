//! Machines group models

use super::{games::VersionGroup, items::Item, moves::Move, resource::NamedApiResource};

/// [Machine official documentation](https://pokeapi.co/docs/v2#machine)
#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Machine {
    /// The identifier for this resource.
    pub id: Option<i64>,
    /// The TM or HM item that corresponds to this machine.
    pub item: Option<NamedApiResource<Item>>,
    /// The move that is taught by this machine.
    #[serde(rename = "move")]
    pub move_: Option<NamedApiResource<Move>>,
    /// The version group that this machine applies to.
    pub version_group: Option<NamedApiResource<VersionGroup>>,
}
