
use crate::abssat::reader::Reader;
use crate::abssat::reader::SetNodesId;

impl Reader {
    //@TODO revisar sería mejor eliminar el nodo selecionado... aunque tenga que hacer una revision...
    pub fn select_and_derive(&mut self) -> Vec<Reader>{
        let mut derive_readers : Vec<Reader> = Vec::new();

        let set_line =  self.gpath.get_set_line(self.current_step);
        for last_selected in set_line {
            let mut reader_copy = self.clone();

            let map_id_node = last_selected.0;
            let mut requires = SetNodesId::new();
            requires.insert(map_id_node);
            reader_copy.last_selected = Some(last_selected);
            reader_copy.last_requires = Some(requires);

            derive_readers.push(reader_copy);
        }
        
        return derive_readers;
    }



}