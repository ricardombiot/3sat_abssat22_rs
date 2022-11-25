
use crate::abssat::gpath::gpow::GPow;
use crate::abssat::gpath::owners::Owners49;
use crate::abssat::utils::alias::{NodeId,PathNodeId, SetNodesId, SetPathNodesId};

impl GPow {
    pub fn do_filter(&mut self, requires : SetNodesId){
        if self.is_valid {
            self.nodes_to_remove = SetPathNodesId::new();

            for map_node_id in requires {
                self.register_filter_nodes_by_require(map_node_id)
            }
        }
        
        if !self.nodes_to_remove.is_empty() {
            self.make_review_owners()
        }
    }

    fn register_filter_nodes_by_require(&mut self, map_node_id_req : NodeId){
        let step_selection = map_node_id_req.0;

        let set_line = self.lines_table.get(&step_selection).unwrap();
        for path_id_node in set_line {
            let is_require = path_id_node.0 == map_node_id_req;
            if !is_require {
                self.nodes_to_remove.insert(*path_id_node);
            }
        }
    }

    fn make_review_owners(&mut self){
        if self.is_valid {
            self.apply_remove_nodes();
            self.clear_invalid_owners();
            self.review_owners_coherence_with_its_parents_sons();

            if !self.nodes_to_remove.is_empty() {
                self.make_review_owners();
            }
        }
    }

    fn apply_remove_nodes(&mut self){
        if self.is_valid {
            for path_id_node in self.nodes_to_remove.drain() {
                let step = path_id_node.0.0;
                let set_line = self.lines_table.get_mut(&step).unwrap();
                set_line.remove(&path_id_node);
                self.owners_table.remove(&path_id_node);
                self.owners.pop(path_id_node);
                
                self.is_valid = self.owners.is_valid();

                if !self.is_valid {
                    break;
                }
            }
        }
    }

    fn clear_invalid_owners(&mut self){
        if self.is_valid {
            for step in 0..self.current_step {
                let set_line = self.lines_table.get(&step).unwrap();
                for path_id_node in set_line {
                    let owners_node = self.owners_table.get_mut(&path_id_node).unwrap();
                    owners_node.intersect(&self.owners);

                    if !owners_node.is_valid() {
                        self.nodes_to_remove.insert(*path_id_node);
                    }
                }
            }
        }
    }

    fn review_owners_coherence_with_its_parents_sons(&mut self){
        if self.is_valid {
            self.review_owners_parents_sons();
            self.review_owners_sons_parents();
        }
    }
    
    /*
    Los owners deben ser coherentes con sus padres e hijos
        # Top to down
        # hago la union de los owners de mis padres y la intersectiono conmigo
    */
    fn review_owners_parents_sons(&mut self){
        if self.is_valid {
            for step in 1..self.current_step {
                let set_line = self.lines_table.get(&step).unwrap();
                for path_id_node in set_line {
                    if !self.is_pending_to_remove(*path_id_node){
                        let parents = self.get_node_parents_owners(*path_id_node);
                        let have_parents = !parents.is_empty();
                        if have_parents {
                            let mut union_owners_parents = Owners49::new();
                            for node_id_parent in parents{
                                let owners_parent = self.get_node_owners(node_id_parent).unwrap();
                                union_owners_parents.union(owners_parent);
                            }

                            let owners_node = self.owners_table.get_mut(&path_id_node).unwrap();
                            owners_node.intersect(&union_owners_parents);

                            if !owners_node.is_valid() {
                                self.nodes_to_remove.insert(*path_id_node);
                            }
                        }else{
                            self.nodes_to_remove.insert(*path_id_node);
                        }
                    }
                }
            }
        }
    }

    /*
        #=
    Los owners deben ser coherentes con sus padres e hijos

    down to top: union de owners de mis hijos intersect with me...
    =#
    */
    fn review_owners_sons_parents(&mut self){
        if self.is_valid {
            let mut step = self.current_step-2;

            while step >= 0 {
                let set_line = self.lines_table.get(&step).unwrap();
                for path_id_node in set_line {
                    if !self.is_pending_to_remove(*path_id_node){
                        let sons = self.get_node_sons_owners(*path_id_node);
                        let have_sons = !sons.is_empty();
                        if have_sons {
                            let mut union_owners_sons = Owners49::new();
                            for node_id_son in sons {
                                let owners_son = self.get_node_owners(node_id_son).unwrap();
                                union_owners_sons.union(owners_son);
                            }

                            let owners_node = self.owners_table.get_mut(&path_id_node).unwrap();
                            owners_node.intersect(&union_owners_sons);

                            if !owners_node.is_valid() {
                                self.nodes_to_remove.insert(*path_id_node);
                            }
                        }else{
                            self.nodes_to_remove.insert(*path_id_node);
                        }
                    }
                }

                step-=1;
            }
        }
    }

    fn is_pending_to_remove(&self, path_id_node : PathNodeId) -> bool {
        return self.nodes_to_remove.contains(&path_id_node)
    }

}