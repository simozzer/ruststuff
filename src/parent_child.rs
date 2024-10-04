use crate::{item_names, ItemNames, ParentChildIds};

pub struct ParentChild {
    pub item_id: i32,
    pub children: Vec<ParentChild>
}


impl ParentChild {

    pub fn create_parent_child(id: i32) -> ParentChild {
        ParentChild{item_id:id, children:Vec::new()}
    }
    
    pub fn add_item(&mut self, child: ParentChild) {
        self.children.push(child);
    }

    pub fn print(&mut self, s_prefix: String) {
        let i = self.item_id;
        println!("{s_prefix},{i}");
        let mut new_prefix = s_prefix;
        new_prefix.push(' ');
        new_prefix.push(' ');
        new_prefix.push(' ');
        for item in self.children.iter_mut() {
            item.print(new_prefix.clone());
        }
    }

    pub fn print_names(&mut self, s_prefix: String, item_names: &mut ItemNames) {
        let i = self.item_id;
        let name = item_names.get_name(i);
        println!("{i}{s_prefix}{name}");
        let mut new_prefix = s_prefix;
        new_prefix.push_str("  ");
        for item in self.children.iter_mut() {
            item.print_names(new_prefix.clone(),item_names);
        }
    }



    pub fn build(&mut self, parent_child_ids: &mut ParentChildIds) {
        let child_ids = parent_child_ids.get_child_ids(self.item_id);
        for child_id in child_ids {
            let mut child = ParentChild{item_id: child_id, children:Vec::new()};
           
            child.build(parent_child_ids);
            self.add_item(child);
        }
    }

    pub fn get_sub_item_ids(&mut self) -> Vec<i32> {
        let mut id_list= Vec::new();
        self._fetch_sub_item_ids(&mut id_list);
        id_list
    }

    pub fn get_leaf_item_ids(&mut self) -> Vec<i32> {
        let mut id_list= Vec::new();
        self._fetch_leaf_item_ids(&mut id_list);
        id_list
    }

    fn _fetch_sub_item_ids(&mut self, id_list: &mut Vec<i32>) {
        for item in self.children.iter_mut() {
            id_list.push(item.item_id);
            item._fetch_sub_item_ids(id_list);
        }
    }

    fn _fetch_leaf_item_ids(&mut self, id_list: &mut Vec<i32>) {
        for item in self.children.iter_mut() {
            if item.children.len() == 0 {
                id_list.push(item.item_id);
            }
            item._fetch_leaf_item_ids(id_list);
        }
    }

}


impl std::fmt::Display for ParentChild {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} has {} children", self.item_id, self.children.len())
    }
}


