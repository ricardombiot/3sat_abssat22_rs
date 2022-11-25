use crate::abssat::{gpath::{gpow::GPow, path_diagram::PathDiagram}, utils::alias::{SetNodesId}};


/*
     x=0 x=1
    !x=1 !x=0
     y=0  y=1
    !y=1  !y=0
*/
#[test]
fn test_join_gpow(){
    let mut gpath_x0 = GPow::new();
    gpath_x0.do_up((0,0));
    gpath_x0.do_up((1,1));

    let mut gpath_x1 = GPow::new();
    gpath_x1.do_up((0,1));
    gpath_x1.do_up((1,0));


    let mut gpath_x0_y0 = gpath_x0.clone();
    let mut gpath_x1_y0 = gpath_x1.clone();

    let mut gpath_x0_y1 = gpath_x0.clone();
    let mut gpath_x1_y1 = gpath_x1.clone();

    gpath_x0_y0.do_up((2,0));
    gpath_x1_y0.do_up((2,0));

    gpath_x0_y1.do_up((2,1));
    gpath_x1_y1.do_up((2,1));

    let mut gpath_join_y0 = gpath_x0_y0;
    gpath_join_y0.do_join(&gpath_x1_y0);
    gpath_join_y0.do_up((3,1));

    let mut gpath_join_y1 = gpath_x0_y1;
    gpath_join_y1.do_join(&gpath_x1_y1);
    gpath_join_y1.do_up((3,0));

    let mut diagram = PathDiagram::new(&gpath_join_y0);
    diagram.build_diagram();
    diagram.to_png("gpath_y0", "test_visual");

    // Step 4 W

    let mut gpath_y0_w0 = gpath_join_y0.clone();
    let mut gpath_y0_w1 = gpath_join_y0.clone();

    gpath_y0_w0.do_up((4, 0));
    gpath_y0_w1.do_up((4, 1));

    let mut gpath_y1_w0 = gpath_join_y1.clone();
    let mut gpath_y1_w1 = gpath_join_y1.clone();

    gpath_y1_w0.do_up((4, 0));
    gpath_y1_w1.do_up((4, 1));


    let mut gpath_w0 = gpath_y0_w0;
    gpath_w0.do_join(&gpath_y1_w0);

    let mut gpath_w1 = gpath_y0_w1;
    gpath_w1.do_join(&gpath_y1_w1);

    // !w=1
    gpath_w0.do_up((5,1));
    // !w=0
    gpath_w1.do_up((5,0));

    // Fusion Node
    gpath_w0.do_up((6,0));
    gpath_w1.do_up((6,0));

    gpath_w0.do_join(&gpath_w1);


    let mut diagram = PathDiagram::new(&gpath_w0);
    diagram.build_diagram();
    diagram.to_png("fusion", "test_visual");

    let mut gpath_fusion = gpath_w0;
    test_filter(&mut gpath_fusion);
}

fn test_filter(gpath : &mut GPow){
    let mut requires = SetNodesId::new();
    requires.insert((0,0));
    gpath.do_filter(requires);

    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    diagram.to_png("filter_path0_0", "test_visual");
}