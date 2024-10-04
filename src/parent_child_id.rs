use std::{collections::HashSet, error::Error};

use csv::Reader;


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
/// Used to hold the relationships between parents and children in a hierarchical structure
pub struct ParentChildIdPair {
    pub parent_id: i32,
    pub child_id: i32
}

/// Returns the information held in a ParentChildIdPair as a string
impl std::fmt::Display for ParentChildIdPair {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{parent_id: {}, child_id: {}}}", self.parent_id, self.child_id)
    }
}


/// Used to hold a list of ParentChildIdPair items
pub struct ParentChildIds {
    items: Vec<ParentChildIdPair>,
    _sorted: bool
}

impl ParentChildIds {

    /// Constructs an instance of the ParentChildIds structure, with an empty list of items
    pub fn build() -> ParentChildIds {
        ParentChildIds {
            items: Vec::new(),
            _sorted: false
        }
    }

    /// Adds a new unique ParentChildIdPair item to the list of items for the ParentChildIds instance.
    /// If an item is already present for the provided parent_id and child_id then the item is not
    /// added and a message is shown in the console.
    pub fn add_item(&mut self, parent_id: i32, child_id:i32) {    
        let found:Option<usize> = self.get_index(ParentChildIdPair{parent_id: parent_id, child_id});
        match found {
            Some(_i) => println!("Item not unique"),
            None => {
                self.items.push(ParentChildIdPair{parent_id: parent_id, child_id});
            }
        }
    
    }

    /// Returns a ParentIdChildPair item for a given index within the list of items.
    pub fn get_item(&mut self, item_index: usize) -> Option<&ParentChildIdPair> {        
        self.items.get(item_index)
    }

    /// Returns the total length of the list of ParentIdChildPair items.
    pub fn len(&mut self) -> usize {
        self.items.len()
    }


    /// Returns an iterator for the list of ParentChildIdPair items.
    pub fn iter(&mut self) -> std::slice::Iter<ParentChildIdPair> {
        self.items.iter()
    }
    

    /// Returns the index of an item from within the list of items.
    pub fn get_index(&mut self, search_key:ParentChildIdPair) -> Option<usize> {
        self.sort();
        let f = &self.items.binary_search(&search_key);
        match f {
            Ok(i) => Some(*i),
            _ => None
        }    
    }

    /// Sorts the list of items.
    pub fn sort(&mut self) {
        if !self._sorted {
            self.items.sort();
            self._sorted = true;
        }
    }

    /// Returns the id of a parent item for a provided item id.
    pub fn get_parent_id(&mut self, item_id: i32) -> Option<i32> {
        for item in self.iter() {
            if item.child_id == item_id {
                return Some(item.parent_id);
            }
        }
        None
    }

    /// Returns a list of ids for the items which are children of a provided parent id.
    pub fn get_child_ids(&mut self, parent_id: i32) -> Vec<i32> {
        let mut v = Vec::new();
        for item in self.iter() {
            if item.parent_id == parent_id {
                v.push(item.child_id);
            }
        }
        v
    }


    /// Returns a list of ids of all the items which are parents of other items.
    pub fn get_parent_ids(&mut self) -> HashSet<i32> {
        let mut parent_ids = HashSet::new();
        for item in self.iter() {
            parent_ids.insert(item.parent_id);
        }
        parent_ids
    }

    pub fn read_csv(&mut self, file_path:&str) -> Result<(), Box<dyn Error>> {
        let mut rdr = Reader::from_path(file_path)?;
        for result in rdr.records() {
            let record = result?;
            self.add_item(record[0].parse().unwrap(), record[1].parse().unwrap());            
        }
        Ok(())
    }

    pub fn is_parent(&mut self, item_id: i32) -> bool {
        for item in self.iter() {
            if item.parent_id == item_id {
                return true;
            }
        }
        return false;
    }



}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_child_pair_init() {
        let result = ParentChildIdPair{parent_id:32, child_id:12};
        assert_eq!(result.parent_id,32);
        assert_eq!(result.child_id, 12);
    }

    #[test]
    fn test_parent_child_pair_to_string() {
        let item = ParentChildIdPair{parent_id:12,child_id:1};
        let s = item.to_string();
        println!("{item}");
        assert_eq!(s, "{parent_id: 12, child_id: 1}");
    }

    #[test]
    fn test_parent_child_ids_init(){
        let result = ParentChildIds::build();
        assert_eq!(result.items.len(),0);
    }

    #[test]
    fn test_parent_child_ids_items(){
        let mut list = ParentChildIds::build();
        assert_eq!(list.items.len(),0);
        
        list.add_item(1,2);
        assert_eq!(list.items.len(),1);

        list.add_item(1,3);        
        list.add_item(2,4);
        list.add_item(1,5);        
        list.add_item(1,6);
        assert_eq!(list.items.len(),5);


        match list.get_index(ParentChildIdPair{parent_id: 2,child_id:4}) {
            Some(item_index) => {
                let item = list.get_item(item_index).unwrap();
                assert_eq!(item.child_id,4);
            },
            None => assert_eq!(-1,0)
        }


        let parent_id = list.get_parent_id(4).unwrap();
        assert_eq!(parent_id,2);

        assert_eq!(list.get_child_ids(0).len(),0);
        assert_eq!(list.get_child_ids(1).len(),4);

        

    }

    #[test]
    fn test_parent_child_ids_get(){
        let mut list = ParentChildIds::build();       
        list.add_item(1,2);
        list.add_item(1,3);        
        list.add_item(1,4);
        list.add_item(1,5);        
        list.add_item(1,6);

        match list.get_item(3) {
            Some(i) => {
                assert_eq!(i.child_id,5);
            }
            ,None => assert_eq!(-1,0)
        }

        match list.get_item(4) {
            Some(i) => {
                assert_eq!(i.child_id,6);
            }
            ,None => assert_eq!(-1,0)
        }

        match list.get_item(0) {
            Some(i) => {
                assert_eq!(i.child_id,2);
            }
            ,None => assert_eq!(-1,0)
        }

        list.iter().for_each(|x| println!("Item from list {x}"));

        let item_index = list.get_index(ParentChildIdPair{parent_id:1,child_id:2}).unwrap();
        assert_eq!(item_index,0);
    }
}
