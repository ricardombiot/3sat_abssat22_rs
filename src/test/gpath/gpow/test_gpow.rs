use crate::abssat::{gpath::gpow::GPow, utils::alias::{PathNodeId, create_new_path_id}};

#[test]
fn test_init_gpow(){

    let mut gpath = GPow::new();

    test_step0(&mut gpath);
    test_step1(&mut gpath);
    test_step2(&mut gpath);
    //@test gpath.current_step == Step(0)
}

fn test_step0(gpath : &mut GPow){
    assert_eq!(gpath.get_map_parent_id().is_none(), true);
    assert_eq!(gpath.get_current_step(), 0);

    let map_node_id = (0,1);
    let path_node_id_expected = create_new_path_id(map_node_id, None);

    assert_eq!(gpath.is(path_node_id_expected), false);
    gpath.do_up(map_node_id);
    assert_eq!(gpath.is(path_node_id_expected), true);

    assert_eq!(gpath.get_map_parent_id().is_none(), false);
    assert_eq!(gpath.get_map_parent_id().unwrap(), map_node_id);
    assert_eq!(gpath.get_current_step(), 1);
}

fn test_step1(gpath : &mut GPow){

    assert_eq!(gpath.get_map_parent_id().is_some(), true);
    assert_eq!(gpath.get_map_parent_id().unwrap(), (0,1));
    assert_eq!(gpath.get_current_step(), 1);

    let map_node_id = (1,0);
    let path_node_id_expected = create_new_path_id(map_node_id, Some((0,1)));
    
    assert_eq!(gpath.is(path_node_id_expected), false);
    gpath.do_up(map_node_id);
    assert_eq!(gpath.is(path_node_id_expected), true);

    assert_eq!(gpath.get_map_parent_id().unwrap(), map_node_id);
    assert_eq!(gpath.get_current_step(), 2);


    // Testing
    let id_1_0_root = create_new_path_id((0,1), None);
    let id_1_0_0_1 = create_new_path_id((1,0), Some((0,1)));

    let owners_id_1_0 = gpath.get_node_owners(id_1_0_root).unwrap();
    assert_eq!(owners_id_1_0.to_list_step(0), [id_1_0_root]);
    assert_eq!(owners_id_1_0.to_list_step(1), [id_1_0_0_1]);
    let parents_id_1_0 = gpath.get_node_parents_owners(id_1_0_root);
    assert_eq!(parents_id_1_0.into_iter().collect::<Vec<PathNodeId>>(), []);
    let sons_id_1_0 = gpath.get_node_sons_owners(id_1_0_root);
    assert_eq!(sons_id_1_0.into_iter().collect::<Vec<PathNodeId>>(), [id_1_0_0_1]);


    let owners_id_1_0_0_1 = gpath.get_node_owners(id_1_0_0_1).unwrap();
    assert_eq!(owners_id_1_0_0_1.to_list_step(0), [id_1_0_root]);
    assert_eq!(owners_id_1_0_0_1.to_list_step(1), [id_1_0_0_1]);
    let parents_id_1_0_0_1 = gpath.get_node_parents_owners(id_1_0_0_1);
    assert_eq!(parents_id_1_0_0_1.into_iter().collect::<Vec<PathNodeId>>(), [id_1_0_root]);
    let sons_id_1_0_0_1 = gpath.get_node_sons_owners(id_1_0_0_1);
    assert_eq!(sons_id_1_0_0_1.into_iter().collect::<Vec<PathNodeId>>(), []);

}

fn test_step2(gpath : &mut GPow){

    assert_eq!(gpath.get_map_parent_id().unwrap(), (1,0));
    assert_eq!(gpath.get_current_step(), 2);

    let map_node_id = (2,1);
    let path_node_id_expected = create_new_path_id(map_node_id, Some((1,0)));
    
    assert_eq!(gpath.is(path_node_id_expected), false);
    gpath.do_up(map_node_id);
    assert_eq!(gpath.is(path_node_id_expected), true);

    assert_eq!(gpath.get_map_parent_id().unwrap(), map_node_id);
    assert_eq!(gpath.get_current_step(), 3);


    // Testing
    let id_1_0_root = create_new_path_id((0,1), None);
    let id_1_0_0_1 = create_new_path_id((1,0), Some((0,1)));
    let id_2_1_1_0 = create_new_path_id((2,1), Some((1,0)));

    let owners_id_1_0 = gpath.get_node_owners(id_1_0_root).unwrap();
    assert_eq!(owners_id_1_0.to_list_step(0), [id_1_0_root]);
    assert_eq!(owners_id_1_0.to_list_step(1), [id_1_0_0_1]);
    assert_eq!(owners_id_1_0.to_list_step(2), [id_2_1_1_0]);
    let parents_id_1_0 = gpath.get_node_parents_owners(id_1_0_root);
    assert_eq!(parents_id_1_0.into_iter().collect::<Vec<PathNodeId>>(), []);
    let sons_id_1_0 = gpath.get_node_sons_owners(id_1_0_root);
    assert_eq!(sons_id_1_0.into_iter().collect::<Vec<PathNodeId>>(), [id_1_0_0_1]);


    let owners_id_1_0_0_1 = gpath.get_node_owners(id_1_0_0_1).unwrap();
    assert_eq!(owners_id_1_0_0_1.to_list_step(0), [id_1_0_root]);
    assert_eq!(owners_id_1_0_0_1.to_list_step(1), [id_1_0_0_1]);
    assert_eq!(owners_id_1_0_0_1.to_list_step(2), [id_2_1_1_0]);
    let parents_id_1_0_0_1 = gpath.get_node_parents_owners(id_1_0_0_1);
    assert_eq!(parents_id_1_0_0_1.into_iter().collect::<Vec<PathNodeId>>(), [id_1_0_root]);
    let sons_id_1_0_0_1 = gpath.get_node_sons_owners(id_1_0_0_1);
    assert_eq!(sons_id_1_0_0_1.into_iter().collect::<Vec<PathNodeId>>(), [id_2_1_1_0]);


    let owners_id_2_1_1_0 = gpath.get_node_owners(id_2_1_1_0).unwrap();
    assert_eq!(owners_id_2_1_1_0.to_list_step(0), [id_1_0_root]);
    assert_eq!(owners_id_2_1_1_0.to_list_step(1), [id_1_0_0_1]);
    assert_eq!(owners_id_2_1_1_0.to_list_step(2), [id_2_1_1_0]);
    let parents_id_2_1_1_0 = gpath.get_node_parents_owners(id_2_1_1_0);
    assert_eq!(parents_id_2_1_1_0.into_iter().collect::<Vec<PathNodeId>>(), [id_1_0_0_1]);
    let sons_id_2_1_1_0 = gpath.get_node_sons_owners(id_2_1_1_0);
    assert_eq!(sons_id_2_1_1_0.into_iter().collect::<Vec<PathNodeId>>(), []);

}