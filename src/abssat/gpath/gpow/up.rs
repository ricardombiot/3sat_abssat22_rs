

use crate::abssat::gpath::gpow::GPow;
use crate::abssat::utils::alias::{NodeId,SetNodesId, PathNodeId, SetPathNodesId ,create_new_path_id};

impl GPow {
    pub fn do_up_filtering(&mut self, requires : SetNodesId, map_id_node : NodeId){
        self.do_filter(requires);
        self.do_up(map_id_node);
    }

    pub fn do_up(&mut self, map_id_node : NodeId){
        if !self.is_valid {
            return;
        }

        self._add_node_set_owners(map_id_node);
        self.current_step += 1;
        self.map_parent_id = Some(map_id_node);
    }

    fn _add_node_set_owners(&mut self, map_id_node : NodeId) {
        let path_id_node = create_new_path_id(map_id_node,self.map_parent_id);

        let mut initial_set_step = SetPathNodesId::new();
        initial_set_step.insert(path_id_node);

        self.lines_table.insert(self.current_step, initial_set_step);
        self.owners_table.insert(path_id_node, self.owners.clone());

        self._add_as_owner_of_all(path_id_node);
    }

    fn _add_as_owner_of_all(&mut self, path_id_node: PathNodeId) {
        for (_id_node, owners_node) in self.owners_table.iter_mut() {
            owners_node.push(path_id_node);
        }

        self.owners.push(path_id_node);
    }

}