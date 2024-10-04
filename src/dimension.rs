use crate::{ItemNames, ParentChild, ParentChildIds};


pub struct Dimension{
    pub names: ItemNames,
    pub parent_child_ids: ParentChildIds,
    pub root_item: ParentChild
}

impl Dimension {

    pub fn new(dimension_name: String) -> Dimension {
        let item_names = ItemNames::build(dimension_name);
        Dimension{
            names:item_names,
            parent_child_ids:ParentChildIds::build(),
            root_item:ParentChild{item_id:-1,children:Vec::new()}
        }
    }


    pub fn load(&mut self, item_names_filename : &str, parent_child_id_filename: &str){        
        self.load_item_names(item_names_filename);
        self.load_parent_child_ids(parent_child_id_filename);
        self.root_item.build(&mut self.parent_child_ids);
    }

    fn load_item_names(&mut self, item_names_filename : &str) {
        self.names.read_csv(item_names_filename);
    }

    fn load_parent_child_ids(&mut self, parent_child_id_filename: &str) {
        self.parent_child_ids.read_csv(parent_child_id_filename);
    }
}
