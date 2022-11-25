use std::env;
use std::path::Path;
use std::fs;

use crate::abssat::sat_machine::SatMachine;
use std::time::Instant;
use crate::abssat::reader_exp::ReaderExp;
use crate::abssat::reader::Reader;
use std::fs::File;
use std::io;
use std::io::Write;
// mode=exhaustive
pub fn execute() -> io::Result<()> {
    //println!("## 3SatMachine ##");

    let mut args = env::args();
    args.next();
    let mode: Option<String> = args.next();
    let instance_name: Option<String> = args.next();

    if mode.is_some()  {
        let mode = mode.unwrap();
        match mode.as_str() {
            "mode=solver" => mode_solver(instance_name)?,
            "mode=exhaustive" => mode_exhaustive(instance_name)?,
            "mode=all-exhaustive" => mode_all_exhaustive()?,
            "-h" | "mode=help" | "help" | "man" => mode_help(),   
            _ => 
                println!("Require mode argument...")
        }
    }
   
    Ok(())
}

pub fn mode_help(){
    println!("Example: ./abs3sat mode=solver <instance_name.cnf> ");
    println!("      Solve a instance and returns only one soluion (if is SAT).");
    println!("Example: ./abs3sat mode=exhaustive <instance_name.cnf> ");
    println!("      Solve a instance and returns all the posible solutions (if is SAT).");
    println!("Example: ./abs3sat mode=all-exhaustive");
    println!("      Solve all the instances and returns all theirs posible solutions (if is SAT).");
    println!("#Note: Abs3Sat only support .cnf files format. #");
}

pub fn mode_solver(instance_name : Option<String>) -> io::Result<()> {
    if instance_name.is_some() {
        let instance_name = instance_name.unwrap();
        let machine = solver_instance(&instance_name);
        output_one_solution(&machine, &instance_name)?;
    }else{
        println!("Require the name of instance...")
    }

    Ok(())
}

pub fn mode_exhaustive(instance_name : Option<String>) -> io::Result<()> {
    if instance_name.is_some() {
        let instance_name = instance_name.unwrap();
        let machine = solver_instance(&instance_name);
        output_exp_solutions(&machine, &instance_name)?;
    }else{
        println!("Require the name of instance...")
    }

    Ok(())
}

pub fn mode_all_exhaustive() -> io::Result<()> {
    //println!("# MODE: all-exhaustive #");
    let base_path_instances = Path::new("output/instances");
    
    let paths = fs::read_dir(base_path_instances).unwrap();

    for path in paths {

        //let path_instance = path.unwrap().path().display().to_string();
        let instance_name = path.unwrap().file_name().into_string().unwrap();
        println!("Name: {}", instance_name);
        let machine = solver_instance(&instance_name);
        output_exp_solutions(&machine, &instance_name)?;
    }

    Ok(())
}

pub fn solver_instance(instance_name : &String) -> SatMachine {
    let path_instance = get_path_instance(&instance_name);

    let now = Instant::now();
    let machine = SatMachine::run_using_cnf(&path_instance).unwrap();
    let elapsed_time = now.elapsed();
    println!("Instance: {} took {} sec.", instance_name, elapsed_time.as_secs());

    return machine;
}

pub fn output_exp_solutions(machine : &SatMachine, instance_name : &String) -> io::Result<()> {
    let output_file_path = get_output_file(&instance_name);
    let output_file_path = Path::new(&output_file_path);

    let mut output_content : String = String::new();

    if machine.have_solution() {
        let mut reader_exp = ReaderExp::new(&machine);
        reader_exp.read();

        let list_solutions = reader_exp.get_list_solutions();

        //println!("SAT");
        output_content += "SAT\n";
        for solution in list_solutions {
            //println!("{:?}", solution)
            for value_bit in solution {
                if *value_bit {
                    output_content += "1";
                }else{
                    output_content += "0";
                }
            }
            output_content += "\n";
        }
    
        //println!("{:?}",reader_exp.get_list_solutions());
    }else{
        output_content += "UNSAT\n";
    }

    //print!("{}", output_content);

    let mut file = File::create(output_file_path)?;
    file.write_all(output_content.as_bytes())?;
    Ok(())
}

pub fn output_one_solution(machine : &SatMachine, instance_name : &String) -> io::Result<()> {
    let output_file_path = get_output_file(&instance_name);
    let output_file_path = Path::new(&output_file_path);

    let mut output_content : String = String::new();

    if machine.have_solution() {
        let mut reader = Reader::new(&machine);
        reader.read();

        let solution = reader.get_solution();

        output_content += "SAT\n";
        output_content += "EXAMPLE SOLUTION: ";
        for value_bit in solution {
            if *value_bit {
                output_content += "1";
            }else{
                output_content += "0";
            }
        }
        output_content += "\n";
    }else{
        output_content += "UNSAT\n";
    }

    println!("{}", output_content);

    let mut file = File::create(output_file_path)?;
    file.write_all(output_content.as_bytes())?;
    Ok(())
}

pub fn get_path_instance(instance_name : &String) -> String {
    let base_path_instances = format!("output/instances/{}", instance_name);
    return base_path_instances;
}

pub fn get_output_file(instance_name : &String) -> String {
    let base_path_solver_sat_machine_rs = "output/solver_sat_machine_rs";
    let output_file = instance_name.replace(".cnf", ".txt");
    let output_file_path = format!("{}/{}", base_path_solver_sat_machine_rs, output_file);

    return output_file_path;
}