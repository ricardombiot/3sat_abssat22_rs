use crate::abssat::sat_machine::SatMachine;
use crate::abssat::reader::Reader;
use crate::abssat::utils::alias::{PathNodeId,SetNodesId, SolutionVector};


impl Reader {
    pub fn new(machine : &SatMachine) -> Reader {
        let gpath = machine.get_gpath_solution().unwrap().clone();
        let n_vars = machine.get_n_vars();
        let solution : SolutionVector = SolutionVector::new();
        let current_step = 0;
        let last_selected : Option<PathNodeId> = None;
        let last_requires : Option<SetNodesId> = None;
        let is_finished = false;

        return Reader {
            gpath, n_vars, solution,
            current_step, last_selected, last_requires,
            is_finished
        };
    }
}