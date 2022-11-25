pub type Step = i32;

pub type LiteralsGateOr = (i32,i32,i32);
pub type LiteralsGateOrSteps = (Step,Step,Step);

pub type Index = i32;
pub type NodeId = (Step, Index);
pub type PathNodeId = (NodeId, NodeId);
pub type OptionalNodeId = Option<NodeId>;

use std::collections::HashSet;

pub type SetNodesId = HashSet<NodeId>;
pub type SetPathNodesId = HashSet<PathNodeId>;

pub type SolutionVector = Vec<bool>;

pub fn create_new_path_id(id : NodeId, parent_id : OptionalNodeId) -> PathNodeId {
    let parent_id = match parent_id {
        None => (-1,0),
        Some(value_parent_id) => value_parent_id
    };

    let path_node_id : PathNodeId = (id, parent_id);
    return path_node_id;
}

pub fn path_id_as_key(path_id_node : PathNodeId) -> String {
    let destine = map_id_as_key(Some(path_id_node.0));
    let origin = map_id_as_key(Some(path_id_node.1));
    return format!("{}_{}",destine,origin);
}

pub fn map_id_as_key(map_id_node : OptionalNodeId) -> String  {
    if is_root(map_id_node) {
        return "r".to_owned();
    }else{
        let map_id_node = map_id_node.unwrap();
        return format!("k{}_{}",map_id_node.0,map_id_node.1);
    }
}

pub fn is_root(map_id_node : OptionalNodeId) -> bool {
    if map_id_node.is_none() {
        return true;
    }

    let map_id_node = map_id_node.unwrap();    
    let step = map_id_node.0;

    return step < 0;
}