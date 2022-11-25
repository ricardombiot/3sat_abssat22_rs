use std::time::Instant;
use crate::abssat::reader_exp::ReaderExp;
use crate::abssat::sat_machine::SatMachine;
use crate::abssat::gpath::path_diagram::PathDiagram;
use crate::abssat::reader::{self, Reader};

#[test]
fn test_sat_machine_simple(){
    let n_vars = 3;
    
    let now = Instant::now();
    let mut machine = SatMachine::new(n_vars);

    let gpath = machine.get_gpath(0).unwrap();
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("sat_machine_simple_init", "test_visual");

    machine.make_step((1,1,1));
    machine.make_close_step();
 
    let clauses = 1;
    assert_eq!(machine.get_current_step(), (n_vars*2)+1+clauses+1 );
    let elapsed_time = now.elapsed();
    println!("Running build_gpath_sat_machine() took {} seconds.", elapsed_time.as_secs());

    //let gpath = machine.get_gpath(0);
    let gpath = machine.get_gpath_fusion();
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("sat_machine_simple", "test_visual");


    let mut reader = Reader::new(&machine);
    assert_eq!(reader.get_stop_step(),(3*2));

    reader.read();
    println!("SOLUTION: {:?}", reader.get_solution());
}


#[test]
fn test_sat_machine_simple_v4(){
    let n_vars= 4;
    let mut machine = SatMachine::new(n_vars);

    // 1 => true, 2 => true
    // 3 => false, 4 => false
    machine.make_step((1,1,1));
    machine.make_step((2,2,2));
    machine.make_step((1,3,4));
    machine.make_step((2,3,4));
    machine.make_step((-3,-3,-3));
    machine.make_step((-4,-4,-4));
    machine.make_close_step();
 

    let gpath = machine.get_gpath_fusion();
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("sat_machine_simple_v4", "test_visual");

    let mut reader = Reader::new(&machine);
    assert_eq!(reader.get_stop_step(),(n_vars*2));

    reader.read();
    println!("SOLUTION: {:?}", reader.get_solution());

    assert_eq!(*reader.get_solution(), [true, true, false, false].to_vec());

}

#[test]
fn test_sat_machine_or123(){
    let n_vars= 3;
    let mut machine = SatMachine::new(n_vars);

    machine.make_step((1,2,3));
    machine.make_close_step();

    let gpath = machine.get_gpath_fusion();
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("sat_machine_or123", "test_visual");

    let mut reader_exp = ReaderExp::new(&machine);
    reader_exp.read();

    println!("{:?}",reader_exp.get_list_solutions());

}