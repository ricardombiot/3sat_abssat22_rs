use crate::abssat::utils::alias::{PathNodeId};
use crate::abssat::gpath::owners::*;


#[test]
fn test_owners_constructor(){
    let owners = Owners49::new();
    assert_eq!(owners.total_steps(), 0);
}


fn get_id0_1_root() -> PathNodeId {
    let root_id = (-1,0);
    let id1 = (0,1);
    let id0_1_root = (id1,root_id);

    return id0_1_root;
}

fn get_id1_0_0_1() -> PathNodeId {
    let id1 = (0,1);
    let id1_0 = (1,0);
    let id1_0_0_1 = (id1_0,id1);

    return id1_0_0_1;
}

fn get_id1_1_0_1() -> PathNodeId {
    let id1 = (0,1);
    let id1_1 = (1,1);
    let id1_1_0_1 = (id1_1,id1);

    return id1_1_0_1;
}
#[test]
fn test_pushing(){
    let mut owners = Owners49::new();
    assert_eq!(owners.total_steps(), 0);

    let id0_1_root = get_id0_1_root();
    owners.push(id0_1_root);

    let id1_0_0_1 = get_id1_0_0_1();
    owners.push(id1_0_0_1);
    assert_eq!(owners.total_steps(), 2);
}


#[test]
fn test_to_list(){
    let mut owners = Owners49::new();
    assert_eq!(owners.total_steps(), 0);

    let id0_1_root = get_id0_1_root();
    owners.push(id0_1_root);

    let list = owners.to_list_step(0);
    assert_eq!(list, [id0_1_root]);

    let id1_0_0_1 = get_id1_0_0_1();
    let id1_1_0_1 = get_id1_1_0_1();
    owners.push(id1_0_0_1);
    owners.push(id1_1_0_1);

    let list = owners.to_list_step(1);
    assert_eq!(list, [id1_0_0_1, id1_1_0_1]);
}

#[test]
fn test_full_to_list(){
    let mut owners = Owners49::new();

    let mut list_expected  = Vec::new();
    let step_origin_example = 3;
    let step_destine_example = 4;
    for index_origin in 0..7 {
        let id_origin = (step_origin_example,index_origin);
        for index_destine in 0..7 {

            let id_destine = (step_destine_example, index_destine);
            let id_path_node = (id_destine,id_origin);
            owners.push(id_path_node);
            list_expected.push(id_path_node);
        }
    }

    assert_eq!(owners.get_binary(4), "0b00000000000001111111111111111111111111111111111111111111111111");

    let list_result = owners.to_list_step(4);
    assert_eq!(list_result, list_expected);
}


fn build_full_step(step : i32) -> Owners49 {
    let mut owners = Owners49::new();
    let step_origin_example = step-1;
    let step_destine_example = step;
    for index_origin in 0..7 {
        let id_origin = (step_origin_example,index_origin);
        for index_destine in 0..7 {

            let id_destine = (step_destine_example, index_destine);
            let id_path_node = (id_destine,id_origin);
            owners.push(id_path_node);
        }
    }

    return owners;
}


#[test]
fn test_intersect(){
    let mut owners = Owners49::new();
    let id0_1_root = get_id0_1_root();
    owners.push(id0_1_root);

    let mut owners_full_step2 = build_full_step(1);
    owners_full_step2.push(id0_1_root);

    assert_eq!(owners_full_step2.is_valid(), true);
    assert_eq!(owners_full_step2.get_binary(1), "0b00000000000001111111111111111111111111111111111111111111111111");
        

    
    let id1_1_0_1 = get_id1_1_0_1();
    owners.push(id1_1_0_1);
    assert_eq!(owners_full_step2.is(id1_1_0_1), true);
    assert_eq!(owners.is(id1_1_0_1), true);

    owners.intersect(&owners_full_step2);

    assert_eq!(owners_full_step2.is(id1_1_0_1), true);
    assert_eq!(owners.is(id1_1_0_1), true);
    assert_eq!(owners.is_valid(), true);
    assert_eq!(owners_full_step2.is_valid(), true);


    owners_full_step2.intersect(&owners);
    assert_eq!(owners_full_step2.get_binary(1), "0b00000000000000000000000000000000000000000000000000000100000000");

    assert_eq!(owners_full_step2.is(id1_1_0_1), true);
    assert_eq!(owners.is(id1_1_0_1), true);
    assert_eq!(owners.is_valid(), true);
    assert_eq!(owners_full_step2.is_valid(), true);


    owners.pop(id1_1_0_1);
    assert_eq!(owners.is_valid(), false);
}


#[test]
fn test_union(){
    let id0_1_root = get_id0_1_root();
    let mut owners_a = Owners49::new();
    owners_a.push(id0_1_root);

    let mut owners_b = owners_a.clone();

    let id1_0_0_1 = get_id1_0_0_1();
    owners_a.push(id1_0_0_1);
    assert_eq!(owners_a.to_list_step(1), [id1_0_0_1]);

    let id1_1_0_1 = get_id1_1_0_1();
    owners_b.push(id1_1_0_1);
    assert_eq!(owners_b.to_list_step(1), [id1_1_0_1]);


    owners_a.union(&owners_b);
    assert_eq!(owners_a.to_list_step(1), [id1_0_0_1, id1_1_0_1]);
}

#[test]
fn test_pushing_and_pop(){
    let mut owners = Owners49::new();
    assert_eq!(owners.total_steps(), 0);

    let id0_1_root = get_id0_1_root();
    let id1_0_0_1 = get_id1_0_0_1();
    let id1_1_0_1 = get_id1_1_0_1();

    owners.push(id0_1_root);


    assert_eq!(owners.get_binary(1), "0b00000000000000000000000000000000000000000000000000000000000000");
    owners.push(id1_0_0_1);
    assert_eq!(owners.get_binary(1), "0b00000000000000000000000000000000000000000000000000000010000000");
    owners.push(id1_1_0_1);
    assert_eq!(owners.get_binary(1), "0b00000000000000000000000000000000000000000000000000000110000000");

    assert_eq!(owners.is(id1_0_0_1), true);
    assert_eq!(owners.is(id1_1_0_1), true);

    owners.pop(id1_0_0_1);

    assert_eq!(owners.is(id1_0_0_1), false);
    assert_eq!(owners.is(id1_1_0_1), true);
    assert_eq!(owners.get_binary(1), "0b00000000000000000000000000000000000000000000000000000100000000");
    assert_eq!(owners.is_valid(), true);

    owners.pop(id1_1_0_1);
    assert_eq!(owners.is(id1_0_0_1), false);
    assert_eq!(owners.is(id1_1_0_1), false);
    assert_eq!(owners.get_binary(1), "0b00000000000000000000000000000000000000000000000000000000000000");
    // Step 1 pasa a estar vacio... entonces no es valido
    assert_eq!(owners.is_valid(), false);
}