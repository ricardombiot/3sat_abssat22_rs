use crate::abssat::utils::alias::SolutionVector;
use crate::abssat::reader::Reader;
use crate::abssat::sat_machine::SatMachine;

#[derive(Debug, Clone)]
pub struct ReaderExp {
    list_readers : Vec<Reader>,
    list_solutions : Vec<SolutionVector>,
    is_finished : bool 
}


impl ReaderExp {
    pub fn new(machine : &SatMachine) -> ReaderExp {
        let reader_seed = Reader::new(machine);
        let mut list_readers : Vec<Reader> = Vec::new();
        list_readers.push(reader_seed);
        let list_solutions : Vec<SolutionVector> = Vec::new();
        let is_finished = false;

        return ReaderExp { 
            list_readers,
            list_solutions,
            is_finished
        }
    }

    pub fn read(&mut self){
        while !self.is_finished {
            self.read_step();
        }
    }

    pub fn read_step(&mut self){
        let mut is_finished = false;
        let mut next_list_readers : Vec<Reader> = Vec::new();

        for mut reader in self.list_readers.drain(..) {
            for mut derive_reader in reader.select_and_derive() {
                derive_reader.make_step_register_and_filter();
                
                if derive_reader.is_finished() {
                    let solution = derive_reader.get_solution().clone();
                    self.list_solutions.push(solution);

                    is_finished = true;
                }else{
                    next_list_readers.push(derive_reader);
                }
            }
        }

        self.list_readers = next_list_readers;
        self.is_finished = is_finished;
    }

    pub fn get_list_solutions(&self) ->  &Vec<SolutionVector> {
        return &self.list_solutions;
    }

}