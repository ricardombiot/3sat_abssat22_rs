


use std::collections::HashMap;
use crate::abssat::utils::alias::{Step, PathNodeId,SetPathNodesId, OptionalNodeId};
use crate::abssat::gpath::owners::Owners49;

#[derive(Debug, Clone)]
pub struct GPow {
    lines_table: HashMap<Step, SetPathNodesId>,
    owners_table: HashMap<PathNodeId, Owners49>,
    owners : Owners49,
    nodes_to_remove : SetPathNodesId,
    current_step : Step,
    map_parent_id : OptionalNodeId,
   //ew_owners : bool,
    is_valid : bool
}

pub enum PathNodeKind {
    NodeLiteral,
    NodeLiteralNeg,
    NodeFusion,
    NodeOr
}

mod constructor;
mod up;
mod join;
mod getters;
mod filter;
