use crate::abssat::gpath::{gpow::GPow};
use crate::abssat::gpath::path_diagram::PathDiagram;
//use std::path::Path;
//use std::fs;

#[test]
fn test_diagram_gpow() -> std::io::Result<()> {

    let mut gpath = GPow::new();

    gpath.do_up((0,1));
    gpath.do_up((1,0));
    gpath.do_up((2,1));

    let mut diagram = PathDiagram::new(&gpath);
    diagram.build_diagram();
    //println!("{}",diagram.get_dot_txt());
    let dir_visual_test = "test_visual";
    diagram.to_png("test1", dir_visual_test);

    //let dir_visual_test = "./../../../test_visual2";
   // let dir_visual_test = "test_visual";
   // fs::create_dir(dir_visual_test)?;
    //println!("{:?}", std::env::current_exe());
    //diagram.to_png("test1", dir_visual_test);
    //let path = Path::new("./../../../test_visual");
    //diagram.to_png("test1".to_string(), path);

    return Ok(());
}

