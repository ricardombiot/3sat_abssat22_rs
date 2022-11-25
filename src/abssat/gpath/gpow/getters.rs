use crate::abssat::utils::alias::{Step, PathNodeId,SetPathNodesId, OptionalNodeId};
use crate::abssat::gpath::owners::Owners49;
use crate::abssat::gpath::gpow::GPow;

impl GPow {

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn get_map_parent_id(&self) -> &OptionalNodeId {
        return &self.map_parent_id;
    }

    pub fn get_current_step(&self) -> Step {
        return self.current_step;
    }

    pub fn is(&self, path_id_node : PathNodeId) -> bool {
        return self.owners.is(path_id_node);
    }

    pub fn get_set_line(&self, step : Step) -> SetPathNodesId {
        let result = match self.lines_table.get(&step) {
            None => SetPathNodesId::new(),
            Some(set) => set.clone()
        };

        return result;
    }

    pub fn get_node_owners(&self, path_id_node : PathNodeId) -> Option<&Owners49> {
        return self.owners_table.get(&path_id_node);
    }

    pub fn get_node_parents_owners(&self, path_id_node : PathNodeId) -> SetPathNodesId {
        let mut set_parents = SetPathNodesId::new();
        let step_node = path_id_node.0.0;

        if step_node != 0 {
            let back_step = step_node-1;
            let myowners = self.get_node_owners(path_id_node).unwrap();
            
            for path_id_node_parent in self.lines_table.get(&back_step).unwrap() {
                let is_parent = myowners.is(*path_id_node_parent);
                if is_parent {
                    set_parents.insert(*path_id_node_parent);
                }
            }
        }

        return set_parents;
    }

    pub fn get_node_sons_owners(&self, path_id_node : PathNodeId) -> SetPathNodesId {
        let mut set_sons =  SetPathNodesId::new();
        let step_node = path_id_node.0.0;

        if step_node != self.current_step-1 {
            let next_step = step_node+1;
            let myowners = self.get_node_owners(path_id_node).unwrap();
            
            let set_step = self.lines_table.get(&next_step);
            if set_step.is_some(){
                for path_id_node_son in set_step.unwrap() {
                    let is_son = myowners.is(*path_id_node_son);
                    if is_son {
                        set_sons.insert(*path_id_node_son);
                    }
                }
            }
        }

        return set_sons;
    }
}