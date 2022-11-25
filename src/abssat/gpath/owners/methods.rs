use crate::abssat::utils::alias::{PathNodeId};
use crate::abssat::gpath::owners::Owners49;
use std::cmp::max;

impl Owners49 {

    pub fn total_steps(&self) -> i32 {
        return self.total_step;
    }

    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }

    pub fn isempty(&self, step : usize) -> bool {
        return match self.get(step) {
            Some(_value) => false,
            None => true
        }
    }

    pub fn get_binary(&self, step : usize) -> String {
        let value = self.secure_get(step);
        return format!("{:#064b}", value);
    }

    pub fn secure_get(&self, step : usize) -> i64 {
        return match self.get(step) {
            Some(value) => value,
            None => 0
        }
    } 

    pub fn get(&self, step : usize) -> Option<i64> {
        if ! self._existe_step(step)  {
            return None;
        }
            
        return Some(self.table[step]);
    }

    pub fn is(&self, path_id_node : PathNodeId) -> bool {
        let (step, index_write) = Owners49::bit_selection(path_id_node);

        if self._existe_step(step){
            //println!("Exist.. Step:{}, Index:{}", step, index_write);
            let mut mask: i64 = 1;
            mask = mask << index_write;
            let value_copy = self.table[step].clone();  
            let result = value_copy & mask; 
            return result != 0;
        }else{
            return false;
        }  
    }

    pub fn push(&mut self, path_id_node : PathNodeId) {
        let (step, index_write) = Owners49::bit_selection(path_id_node);
        let mut adder: i64 = 1;
        adder = adder << index_write;

        self._add_step_if_dont_exist(step);

        let value = &mut self.table[step] ;
        *value = *value | adder;
    }  

    pub fn pop(&mut self, path_id_node : PathNodeId) {
        let (step, index_write) = Owners49::bit_selection(path_id_node);

        if self._existe_step(step){
            let mut subber: i64 = 1;
            subber = subber << index_write;

            let mut mask: i64 = i64::max_value();
            mask = mask ^ subber;

            let value = &mut self.table[step] ;
            *value = *value & mask;

            self._check_if_isempty(step);
        }
    }

    // apply OR
    pub fn union(&mut self, owners_b : &Owners49){
        if self.is_valid() && owners_b.is_valid() {
            let max_step = max(self.total_steps(), owners_b.total_steps());

            for step in 0..max_step {
                let index_step = step as usize;
                let value_b = owners_b.secure_get(index_step);
                
                if self._existe_step(index_step){
                    let value = &mut self.table[index_step] ;
                    *value = *value | value_b;
                }else{
                    self._add_step_if_dont_exist(index_step);
                    let value = &mut self.table[index_step] ;
                    *value = value_b;
                }
            }
        }
    }

    // AND
    pub fn intersect(&mut self, owners_b : &Owners49){
        if self.is_valid() && owners_b.is_valid() {
            let max_step = max(self.total_steps(), owners_b.total_steps());

            for step in 0..max_step {
                let index_step = step as usize;
                let value_b = owners_b.secure_get(index_step);
                
                if self._existe_step(index_step){
                    let value = &mut self.table[index_step] ;
                    *value = *value & value_b;
                }else{
                    self._add_step_if_dont_exist(index_step);
                    self.is_valid = false;
                }

                self._check_if_isempty(index_step);
            }
        }
    }

    fn _check_if_isempty(&mut self, step : usize) {
        if self.is_valid {
            if self._existe_step(step){
                let value = &self.table[step] ;
                self.is_valid = *value != 0;
            }else{
                self.is_valid = false;
            }
        }
    }

    fn _existe_step(&self, step : usize) -> bool {
        let n_steps = self.total_steps();
        let require_add = (step as i32) - (n_steps-1);
        //println!("Require add {:?}",require_add);
        return !(require_add > 0);
    }

    fn _add_step_if_dont_exist(&mut self, step : usize){
        let n_steps = self.total_steps();
        let require_add = (step as i32) - (n_steps-1);
        //println!("{:?}",require_add);
        if require_add > 0 {
            for _x in 0..require_add {
                self.table.push(0);
                self.total_step += 1;
            }
        }
    }

}