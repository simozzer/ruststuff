use crate::{Dimension, ThreeDimValues};

pub struct Questions
{
    pub dimensions: Vec<Dimension>,
    pub values: ThreeDimValues
}

impl Questions
{
    pub fn ask_question(&mut self, keys: [i32;3]) -> f64 {
        if self.is_group_level(keys) {
            return 0.02;
        }
        return 0.01;
    }

    fn is_group_level(&mut self, keys: [i32;3]) -> bool {
        for (index, val) in keys.iter().enumerate() {  
            println!("index: {}, val:{}", index, val);
            let d = &mut self.dimensions[index];
            let is_group = d.parent_child_ids.is_parent(*val);
            if is_group {
                return true;
            }
        }
        false
    }
}


mod tests {
    use super::*;
    #[test]
    fn test_keys() {
        //
    }
}