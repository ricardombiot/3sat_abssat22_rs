

use crate::abssat::gpath::owners::*;

#[test]
fn test_indexing_from_root(){
    let owners = Owners49::new();
    assert_eq!(owners.total_steps(), 0);

    let root_id = (-1,0);

    let id0 = (0,0);
    let id0_0_root = (id0,root_id);
    let (step, index_write) =  Owners49::bit_selection(id0_0_root);

    assert_eq!(step, 0);
    assert_eq!(index_write, 0);


    let id1 = (0,1);
    let id0_1_root = (id1,root_id);
    let (step, index_write) =  Owners49::bit_selection(id0_1_root);

    assert_eq!(step, 0);
    assert_eq!(index_write, 1);
}


#[test]
fn test_indexing_from_step0(){
    let owners = Owners49::new();
    assert_eq!(owners.total_steps(), 0);

    let id0 = (0,0);

    let id1_1 = (1,1);
    let id1_1_0_0 = (id1_1,id0);
    let (step, index_write) =  Owners49::bit_selection(id1_1_0_0);

    assert_eq!(step, 1);
    assert_eq!(index_write, 1);

    let id1 = (0,1);

    let id1_0 = (1,0);
    let id1_0_0_1 = (id1_0,id1);
    let (step, index_write) =  Owners49::bit_selection(id1_0_0_1);

    assert_eq!(step, 1);
    assert_eq!(index_write, 7);

}


#[test]
fn test_indexing_from_step1(){
    let owners = Owners49::new();
    assert_eq!(owners.total_steps(), 0);

    let id1_0 = (1,0);

    let id2_0 = (2,0);
    let id2_1 = (2,1);

    let id2_0_1_0 = (id2_0,id1_0);
    let (step, index_write) =  Owners49::bit_selection(id2_0_1_0);

    assert_eq!(step, 2);
    assert_eq!(index_write, 0);

    let id2_1_1_0 = (id2_1,id1_0);
    let (step, index_write) =  Owners49::bit_selection(id2_1_1_0);

    assert_eq!(step, 2);
    assert_eq!(index_write, 1);


    let id1_1 = (1,1);


    let id2_0_1_1 = (id2_0,id1_1);
    let (step, index_write) =  Owners49::bit_selection(id2_0_1_1);

    assert_eq!(step, 2);
    assert_eq!(index_write, 7);

    let id2_1_1_1 = (id2_1,id1_1);
    let (step, index_write) =  Owners49::bit_selection(id2_1_1_1);

    assert_eq!(step, 2);
    assert_eq!(index_write, 7+1);

}