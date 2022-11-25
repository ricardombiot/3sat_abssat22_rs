// I need only 7*7 bits for each step

use crate::abssat::utils::alias::{PathNodeId};
use crate::abssat::gpath::owners::Owners49;



impl Owners49 {
    pub fn to_list_step(&self, step : usize) -> Vec<PathNodeId> {
        let mut list : Vec<PathNodeId> = Vec::new();

        let value = self.secure_get(step);
        let inbinary = format!("{:#064b}", value);
        let mut chars = inbinary.chars();

        let step_destine = step as i32;
        let step_origin = step_destine-1;

        let mut index_destine = 0;
        let mut index_origin = 0;
        //let mut counter = 0;
        for _index_writed in 0..49 {
            let bit = chars.next_back().unwrap();
            if bit == '1' {
                //let index_destine = index_writed - (index_origin*7);
                let id_origin = (step_origin, index_origin);
                let id_destine = (step_destine, index_destine);
                let id = (id_destine, id_origin);
                list.push(id);
                //counter +=1;
            }

            index_destine += 1;

            if index_destine == 7 {
                //counter = 0;
                index_destine = 0;
                index_origin += 1;
            }
        }

        return list;
    }

}