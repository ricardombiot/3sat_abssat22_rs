
use crate::abssat::reader::Reader;

use crate::abssat::utils::alias::{SetNodesId};


impl Reader {

    pub fn read(&mut self){
        while !self.is_finished() {
            //println!("Reader step: {}", self.current_step);
            self.read_step()
        }
    }

    pub fn read_step(&mut self){
        if !self.is_finished() {
            self.select_id();
            self.make_step_register_and_filter();
        }
    }

    fn select_id(&mut self){
        let set_line = self.gpath.get_set_line(self.current_step);

        let first_id_step = set_line.into_iter().next().unwrap();
        let map_id_node = first_id_step.0;

        self.last_selected = Some(first_id_step);
        let mut requires = SetNodesId::new();
        requires.insert(map_id_node);
        self.last_requires = Some(requires);
    }

    fn register_selection(&mut self){
        let value_index_bit : i32 = self.last_selected.unwrap().0.1;
        let is_one = value_index_bit == 1;
        if is_one {
            self.solution.push(true);
        }else{
            self.solution.push(false);
        }
    }

    fn filter_gpath(&mut self){
        let requires  = self.last_requires.as_ref().unwrap().clone();
        self.gpath.do_filter(requires);
    }


    pub fn make_step_register_and_filter(&mut self){
        self.register_selection();
        self.filter_gpath();
        self.current_step += 2;

        if self.is_stop_step() {
            self.is_finished = true;
        }
    }

    fn is_stop_step(&self) -> bool {
        return self.current_step >= self.get_stop_step();
    }
}