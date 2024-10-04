
use std::cmp::Ordering;
use crate::ValueKeys; 

pub struct ThreeDimValue {
    pub key: ValueKeys,
    pub value: f64
}


impl std::fmt::Display for ThreeDimValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.key, self.value)
    }
}

pub struct ThreeDimValues {
    pub items: Vec<ThreeDimValue>,
    pub _sorted: bool
}

impl ThreeDimValues {

    pub fn add_item(&mut self, keyed_value: ThreeDimValue) {    
        let found:Option<usize> = self.get_index(&keyed_value);
        match found {
            Some(_i) => println!("Item not unique, so not added"),
            None => {
                self.items.push(keyed_value);
            }
        }    
    }

    pub fn set_value(&mut self, keyed_value: ThreeDimValue) {    
        let found:Option<usize> = self.get_index(&keyed_value);
        match found {
            Some(i) => {
                let item = &mut self.items[i];
                item.value = keyed_value.value;
            }
            None => {
                self.items.push(keyed_value);
            }
        }    
    }
    
    pub fn delete_value(&mut self, keyed_value: ThreeDimValue) {    
        let found:Option<usize> = self.get_index(&keyed_value);
        match found {
            Some(i) => {
                let _ = &self.items.remove(i);
            }
            None => {
                println!("Value not found, so not deleted");
            }
        }    
    }

    /// Returns a ParentIdChildPair item for a given index within the list of items.
    pub fn get_item(&mut self, item_index: usize) -> Option<&ThreeDimValue> {        
        self.items.get(item_index)
    }

    /// Returns the total length of the list of ParentIdChildPair items.
    pub fn len(&mut self) -> usize {
        self.items.len()
    }


    /// Returns an iterator for the list of ParentChildIdPair items.
    pub fn iter(&mut self) -> std::slice::Iter<ThreeDimValue> {
        self.items.iter()
    }
    

    /// Returns the index of an item from within the list of items.
    pub fn get_index(&mut self, search_key:&ThreeDimValue) -> Option<usize> {
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
}


impl Ord for ThreeDimValue {

    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for ThreeDimValue {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)       
    }

}

impl PartialEq for ThreeDimValue {

    fn eq(&self, other: &ThreeDimValue) -> bool {
        return self.key == other.key;
    }
}

impl Eq for ThreeDimValue {

}

