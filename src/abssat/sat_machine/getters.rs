use crate::abssat::gpath::gpow::GPow;
use crate::abssat::utils::alias::Step;
use crate::abssat::sat_machine::SatMachine;

impl SatMachine {

    pub fn is_valid(&self) -> bool {
       return self.is_valid;
    }

    pub fn get_n_vars(&self) -> i32 {
        return self.n_vars;
    }

    pub fn have_solution(&self) -> bool {
        return self.is_valid && self.is_close;
    }

    pub fn get_gpath_solution(&self) -> Option<&GPow> {
        if self.have_solution() {
            return Some(self.get_gpath_fusion());
        }else{
            return None;
        }
    }

    pub fn get_current_step(&self) -> Step {
        return self.current_step;
    }

    pub fn get_gpath(&self, index : usize) -> Option<&GPow> {
        return self.timeline[index].as_ref();
    }

    pub fn get_gpath_fusion(&self) -> &GPow {
       return self.timeline[0].as_ref().unwrap()
    }

}