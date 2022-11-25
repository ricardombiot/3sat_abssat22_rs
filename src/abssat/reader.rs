use crate::abssat::gpath::gpow::GPow;
use crate::abssat::utils::alias::{Step,PathNodeId,SetNodesId, SolutionVector};


#[derive(Debug, Clone)]
pub struct Reader {
    gpath : GPow,
    n_vars : i32,
    solution : SolutionVector,
    current_step : Step,
    last_selected : Option<PathNodeId>,
    last_requires : Option<SetNodesId>,
    is_finished : bool
}


mod constructor;
mod read;
mod getters;

mod derive;