

use crate::abssat::sat_machine::{SatMachine, init_timeline, Timeline};
use crate::abssat::utils::alias::{LiteralsGateOr,LiteralsGateOrSteps};
use crate::abssat::sat_machine::GPow;

use super::OptionalGPow;

impl SatMachine {

    pub fn make_step(&mut self, literals : LiteralsGateOr) {
        if self.is_valid {

            let literals_steps = SatMachine::literals_to_steps(literals);
            let mut is_valid = false;
            let mut next_step_timeline = init_timeline();
            
            for index_tl in 0..7 {
                let cell_tl = self.timeline[index_tl].as_ref();
                //println!("Step: {} Cell: {}", self.current_step,index_tl);
                if cell_tl.is_some() {
                    //println!("Is some..");
                    is_valid = true;

                    let gpath = cell_tl.unwrap();
                    let first_gpath_copy = gpath.clone();
                    self.do_step(&mut next_step_timeline, first_gpath_copy, literals_steps);
                }
            }
            
         
            self.timeline = next_step_timeline;
            self.current_step += 1;

            self.is_valid = is_valid;
        }
    }

    fn do_step(&mut self, next_step_timeline : &mut Timeline, gpath : GPow, literals_steps : LiteralsGateOrSteps){
        let mut case_gpaths = Vec::new();
        
        let gpath001 = gpath.clone();
        let case_gpath001 = (0, (0,0,1), gpath001);
        case_gpaths.push(case_gpath001);

        let gpath010 = gpath.clone();
        let case_gpath010 = (1, (0,1,0), gpath010);
        case_gpaths.push(case_gpath010);

        let gpath011 = gpath.clone();
        let case_gpath011 = (2,(0,1,1), gpath011);
        case_gpaths.push(case_gpath011);

        let gpath100 = gpath.clone();
        let case_gpath100 = (3,(1,0,0), gpath100);
        case_gpaths.push(case_gpath100);

        let gpath101 = gpath.clone();
        let case_gpath101 = (4,(1,0,1), gpath101);
        case_gpaths.push(case_gpath101);

        let gpath110 = gpath.clone();
        let case_gpath110 = (5,(1,1,0), gpath110);
        case_gpaths.push(case_gpath110);
        // this dirty code is for save one clone expensive operation..
        let gpath111 = gpath;
        let case_gpath111 = (6,(1,1,1), gpath111);
        case_gpaths.push(case_gpath111);

        for (case_index ,case_indexes, mut gpath) in case_gpaths.drain(..) {
            let requires = SatMachine::build_requires(literals_steps, case_indexes);
            //println!("{:?}",requires);
            let map_id_node = (self.current_step, case_index);
            gpath.do_up_filtering(requires, map_id_node);
            SatMachine::do_impact(next_step_timeline, gpath, case_index);
        }
    }

    fn do_impact(next_step_timeline : &mut Timeline, gpath : GPow, case_index : i32){
        if gpath.is_valid(){
            let case_index : usize = case_index as usize;
            //println!("Impact on {}..", case_index);
            match &mut next_step_timeline[case_index] {
                None => {
                    next_step_timeline[case_index] = Some(gpath);
                },
                Some(gpath_joined) => {
                    gpath_joined.do_join(&gpath);
                    drop(gpath);
                }
            }
        }
        /*else{
            println!("Impact on INVALID {}..", case_index);
        }*/
    }


    pub fn make_close_step(&mut self) {
        let mut fusion_gpath : OptionalGPow = None;

        if self.is_valid { 
            let mut is_valid = false;
            for index_tl in 0..7 {
                let cell_tl = self.timeline[index_tl].as_mut();
                //println!("Step: {} Cell: {}", self.current_step,index_tl);
                if cell_tl.is_some() {
                    //println!("Some for fusion...");
                    is_valid = true;
    
                    let gpath = cell_tl.unwrap();
                    gpath.do_up((self.current_step,0));
                    
                    if fusion_gpath.is_none() {
                        fusion_gpath = Some(gpath.clone());
                    }else{
                        let gpath_joined = fusion_gpath.as_mut().unwrap();
                        gpath_joined.do_join(&gpath);
                    }
                }
            }

            self.timeline = init_timeline();
            self.timeline[0] = fusion_gpath;
            //println!("{:?}", self.timeline[0]);
            self.current_step += 1;
            self.is_valid = is_valid;
            self.is_close = true;
        }
        /*else{
            println!("invalid.. make step fusion");
        }*/
    }


}
