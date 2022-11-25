

use crate::abssat::{reader::Reader, sat_machine::SatMachine};

#[test]
fn test_sat_machine_cnf_import(){
    println!("test_sat_machine_cnf_import");
    let machine = SatMachine::run_using_cnf("test_examples/basic_v3_c1.cnf").unwrap();

    assert_eq!(machine.is_valid(),true);
    assert_eq!(machine.have_solution(),true);

    let mut reader = Reader::new(&machine);
    assert_eq!(reader.get_stop_step(),(3*2));

    reader.read();
    println!("SOLUTION: {:?}", reader.get_solution());
}

#[test]
fn test_rand3sat_v4_c20(){
    let machine = SatMachine::run_using_cnf("test_examples/rand3sat_v4_c20.cnf").unwrap();

    assert_eq!(machine.is_valid(),true);
    assert_eq!(machine.have_solution(),true);
}

#[test]
fn test_rand3sat_v8_c10(){
    let machine = SatMachine::run_using_cnf("test_examples/rand3sat_v8_c10.cnf").unwrap();

    assert_eq!(machine.is_valid(),true);
    assert_eq!(machine.have_solution(),true);
}

#[test]
fn test_unsat_v1_c2(){
    let machine = SatMachine::run_using_cnf("test_examples/unsat_v1_c2.cnf").unwrap();

    assert_eq!(machine.is_valid(),false);
    assert_eq!(machine.have_solution(),false);
}

