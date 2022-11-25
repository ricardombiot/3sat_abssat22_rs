
use crate::abssat::utils::alias::{SetNodesId, LiteralsGateOr,LiteralsGateOrSteps};
use crate::abssat::sat_machine::Step;
use crate::abssat::sat_machine::SatMachine;

impl SatMachine {

    pub fn literals_to_steps(literals : LiteralsGateOr) -> LiteralsGateOrSteps {
        let a = SatMachine::literal_to_step(literals.0);
        let b = SatMachine::literal_to_step(literals.1);
        let c = SatMachine::literal_to_step(literals.2);

        return (a,b,c);
    }

    fn literal_to_step(val : i32) -> Step {
        let is_neg = val < 0;
        let mut step = (i32::abs(val)-1)*2;
        
        if is_neg {
            step+= 1;
        }

        return step;
    }

    pub fn build_requires( literals_steps : LiteralsGateOrSteps, case_indexes : (i32, i32, i32) ) -> SetNodesId {
        let (a_step, b_step, c_step) = literals_steps;
        let (a_index, b_index, c_index) = case_indexes;
        
        let mut requires = SetNodesId::new();
        requires.insert((a_step,a_index));
        requires.insert((b_step,b_index));
        requires.insert((c_step,c_index));

        return requires;
    }

}