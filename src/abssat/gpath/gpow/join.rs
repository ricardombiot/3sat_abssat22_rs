use crate::abssat::gpath::gpow::GPow;
//use crate::abssat::utils::alias::{NodeId,PathNodeId, SetPathNodesId ,create_new_path_id};

impl GPow {

    pub fn do_join(&mut self, gpath_inmutable : &GPow){
        if GPow::is_valid_join(self, gpath_inmutable) {

            self.owners.union(&gpath_inmutable.owners);
            let max_step = self.current_step;
            for step in 0..max_step {
                for path_id_node in self.owners.to_list_step(step as usize) {
                    let set_line = self.lines_table.get_mut(&step);
                    set_line.unwrap().insert(path_id_node);

                    let set_owners_node_a = self.owners_table.get_mut(&path_id_node);
                    let set_owners_node_b = gpath_inmutable.owners_table.get(&path_id_node);

                    if set_owners_node_a.is_none() {
                        self.owners_table.insert(path_id_node, set_owners_node_b.unwrap().clone());
                    }else if set_owners_node_b.is_some() {
                        let set_owners_node_a =  set_owners_node_a.unwrap();
                        let set_owners_node_b =  set_owners_node_b.unwrap();
                        
                        set_owners_node_a.union(set_owners_node_b);
                    }
                }
            }

        }
    }

    fn is_valid_join(gpath : &GPow, gpath_inmutable : &GPow) -> bool{
        let eq_map_parent_id = gpath.map_parent_id == gpath_inmutable.map_parent_id;
        let eq_current_step = gpath.current_step == gpath_inmutable.current_step;
        let both_valids = gpath.is_valid && gpath_inmutable.is_valid;
    
        return eq_map_parent_id && eq_current_step && both_valids
    }
}