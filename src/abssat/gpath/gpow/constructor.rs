
use std::collections::HashMap;
use crate::abssat::utils::alias::{Step, PathNodeId,SetPathNodesId, OptionalNodeId};
use crate::abssat::gpath::owners::Owners49;
use crate::abssat::gpath::gpow::GPow;

impl GPow {
    pub fn new() -> Self { 
        let lines_table : HashMap<Step, SetPathNodesId> = HashMap::new();
        let owners_table: HashMap<PathNodeId, Owners49>  = HashMap::new();
        let owners: Owners49 = Owners49::new();
        let nodes_to_remove : SetPathNodesId = SetPathNodesId::new(); 
        let current_step: Step = 0;
        let map_parent_id: OptionalNodeId = None;
 
        let is_valid: bool = true;

        Self {
            lines_table, 
            owners_table,
            owners,
            nodes_to_remove,
            current_step,
            map_parent_id,
            is_valid
        }
    }
}
