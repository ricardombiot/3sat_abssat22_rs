// I need only 7*7 bits for each step

use crate::abssat::utils::alias::{PathNodeId};
use crate::abssat::gpath::owners::Owners49;

impl Owners49 {

    pub fn bit_selection(path_id_node : PathNodeId) -> (usize,usize) {
        let step_destine = path_id_node.0.0;
        let index_destine = path_id_node.0.1;
        //let step_origin = path_id_node.1.0;
        let index_origin = path_id_node.1.1;

        let index = (index_origin*7) + index_destine;

        return (step_destine as usize, index as usize);
    }
}