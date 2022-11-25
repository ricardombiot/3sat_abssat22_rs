
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use crate::abssat::gpath::path_diagram::PathDiagram;

impl<'a> PathDiagram<'a> {

    pub fn to_png(&self, name : &str, path_str : &str){
        let input_file = format!("{}/{}.dot", path_str,name);
        let output_file = format!("{}/{}.png", path_str,name);

        let path_input_file = Path::new(&input_file);
        let display = path_input_file.display();
        //let path_output_file = Path::new(&output_file);

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path_input_file) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(self.dot_txt.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }

        let mut cmd_dot = Command::new("dot");
        cmd_dot.arg("-Tpng").arg(input_file).arg("-o").arg(output_file);

        cmd_dot.output().expect("failed to execute process");
    }

}