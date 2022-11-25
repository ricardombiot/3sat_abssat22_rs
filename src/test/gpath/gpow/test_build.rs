use std::collections::HashMap;
use crate::abssat::gpath::gpow::GPow;
use crate::abssat::gpath::path_diagram::PathDiagram;
use crate::abssat::sat_machine::SatMachine;
use std::time::Instant;

fn init_graph(n_vars : i32) -> (i32,GPow) {
    let mut current_step = 0;
    //let n_vars = 3;

    let mut timeline : HashMap<i32, GPow> = HashMap::new();
    let mut gpath_init0 = GPow::new();
    gpath_init0.do_up((0,0));
    gpath_init0.do_up((1,1));
    let mut gpath_init1 = GPow::new();
    gpath_init1.do_up((0,1));
    gpath_init1.do_up((1,0));

    
    // Step 2
    timeline.insert(1, gpath_init0);
    timeline.insert(0, gpath_init1);
    current_step += 2;

    
    for _i in 1..n_vars {
       let gpath0 = timeline.get_mut(&0).unwrap();
       let mut gpath0_to_0 = gpath0.clone();
       let mut gpath0_to_1 = gpath0.clone();

       let gpath1 = timeline.get_mut(&1).unwrap();
       let mut gpath1_to_0 = gpath1.clone();
       let mut gpath1_to_1 = gpath1.clone();

       gpath0_to_0.do_up((current_step,0));
       gpath1_to_0.do_up((current_step,0));

       let mut gpath_on0joined = gpath0_to_0;
       gpath_on0joined.do_join(&gpath1_to_0);

       gpath0_to_1.do_up((current_step,1));
       gpath1_to_1.do_up((current_step,1));

       let mut gpath_on1joined = gpath0_to_1;
       gpath_on1joined.do_join(&gpath1_to_1);

       current_step += 1;
       // Negados
       gpath_on0joined.do_up((current_step,1));
       gpath_on1joined.do_up((current_step,0));

       timeline.insert(1, gpath_on0joined);
       timeline.insert(0, gpath_on1joined);
       current_step += 1;
    }

    let gpath0 = timeline.get_mut(&0).unwrap();
    let mut gpath0_to_fusion = gpath0.clone();

    let gpath1 = timeline.get_mut(&1).unwrap();
    let mut gpath1_to_fusion = gpath1.clone();

    gpath0_to_fusion.do_up((current_step,0));
    gpath1_to_fusion.do_up((current_step,0));

    let mut gpath_fusion = gpath0_to_fusion;
    gpath_fusion.do_join(&gpath1_to_fusion);

    current_step += 1;

    return (current_step, gpath_fusion);
}

#[test]
fn test_build_init_gpath(){
    // IMPORTANT:
    // EXECUTE WITH RELEASE MODE
    // cargo test --release -- --nocapture
    let n_vars = 3;

    let now = Instant::now();
    let (current_step, gpath) = init_graph(n_vars);
 
    assert_eq!(gpath.get_current_step(), current_step);
    let elapsed_time = now.elapsed();
    println!("Running build_init_gpath() took {} seconds.", elapsed_time.as_secs());
    
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("build_nvars3", "test_visual");
}

#[test]
fn test_build_gpath_sat_machine(){
    // IMPORTANT:
    // EXECUTE WITH RELEASE MODE
    // cargo test --release -- --nocapture
    let n_vars = 3;
    
    let now = Instant::now();
    let machine = SatMachine::new(n_vars);
 
    assert_eq!(machine.get_current_step(), (n_vars*2)+1 );
    let elapsed_time = now.elapsed();
    println!("Running build_gpath_sat_machine() took {} seconds.", elapsed_time.as_secs());

    let gpath = machine.get_gpath_fusion();
    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("build_nvars3_machine", "test_visual");
}