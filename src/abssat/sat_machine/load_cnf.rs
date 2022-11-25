

use crate::abssat::sat_machine::SatMachine;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

impl SatMachine {

    pub fn run_using_cnf(file_path : &str) -> Option<SatMachine> {
        let file_path = Path::new(&file_path);

        match File::open(file_path) {
            Ok(file) => {
                return SatMachine::build_using_cnf_file(file)
            },
            _ => {
                panic!("Couldnt open .cnf file. Review the path {}", file_path.display());
                //return None;
            }
        }
    }

    fn build_using_cnf_file(file : File) -> Option<SatMachine> {
        let mut machine : Option<SatMachine> = None;
        let mut cursor_buffer = BufReader::new(file);
        let mut line = String::new();

        loop {
            let bytes = cursor_buffer.read_line(&mut line).expect("Reading line, error...");
            let is_eof = line == "" && bytes == 0;

            if is_eof {
                break;
            }else{
                
                let ch = line.chars().next().unwrap();
                match ch {
                    'c' => {}
                    'p' => {
                        machine = SatMachine::init_reading_cnf_config(&line);
                    }
                    _ => {
                        if machine.is_some() {
                            let can_have_solution = machine.as_mut()
                                .unwrap()
                                .make_step_reading_clause(&line);

                            if !can_have_solution {
                                break;
                            }
                        }
                    }
                }      
            }  

            line.clear();
        }

        if machine.is_some() {
            machine.as_mut().unwrap().make_close_step();
        }

        return machine;
    }

    fn make_step_reading_clause(&mut self, line : &String) -> bool{
        let mut values = line.split(" ")
            .take(3)
            .map( |literal| 
                literal.to_string().parse::<i32>().unwrap()
            );

        let a = values.next().unwrap();
        let b = values.next().unwrap();
        let c = values.next().unwrap();
        let literals = (a,b,c);
        //println!("READ LITERALS: {:?}", literals);
        self.make_step(literals);

        return self.is_valid();
    }

    fn init_reading_cnf_config(line : &String) -> Option<SatMachine> {
        //println!("CONFIGURATION");
        let mut words = line.split(" ");
        words.next();
        if words.next().unwrap() == "cnf" {
            let n_vars_txt = words.next().unwrap().to_string();
            let n_vars = n_vars_txt.parse::<i32>().unwrap();
            //println!("Initialize machine with {} n_vars", n_vars);
            let machine = SatMachine::new(n_vars);

            return Some(machine);
        }else{
            panic!("Fail format. We only support .cnf");
        }

        //return None;
    }

    

    /* 
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
            let file = File::open(filename)?;

            let mut cursor_buffer = BufReader::new(file);
            cursor_buffer.read_line(buf);
            Ok(io::BufReader::new(file).lines())
    }*/
}