use crate::abssat::reader::Reader;
use crate::abssat::utils::alias::{Step, SolutionVector};

impl Reader {

    pub fn get_stop_step(&self) -> Step {
        return self.n_vars*2 as Step;
    }

    pub fn get_solution(&self) -> &SolutionVector {
        return &self.solution;
    }
    
    pub fn is_finished(&self) -> bool {
        return self.is_finished
    }

}