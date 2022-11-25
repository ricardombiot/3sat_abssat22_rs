
use crate::abssat::gpath::gpow::GPow;
//use crate::abssat::utils::alias::{PathNodeId, Step, path_id_as_key};

pub struct PathDiagram<'a> {
    graph : &'a GPow,
    dot_txt : String
}

impl<'a> PathDiagram<'a> {
    pub fn new(graph : &'a GPow) -> PathDiagram {
        let dot_txt = String::new();
        Self {graph, dot_txt}
    }

    pub fn get_dot_txt(&self) -> &String {
        return &self.dot_txt
    }
}

mod build_diagram;
mod export_diagram;
