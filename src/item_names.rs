use csv::{Reader};
use std::error::Error;

pub struct ItemName{
    item_id: i32,
    item_name: String
}


pub struct ItemNames {
    items: Vec<ItemName>,
    _sorted: bool
}


impl ItemNames {

    pub fn build(type_name: String) -> ItemNames {
        let mut item_names = Vec::new();
        item_names.push(ItemName{item_id:-1, item_name:type_name});
        ItemNames {
            items: item_names,
            _sorted: false
        }        
    }


    pub fn read_csv(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr: Reader<std::fs::File> = Reader::from_path(file_path)?;
        for result in rdr.records() {
            let record = result?;
            self.items.push(ItemName{item_id:record[0].parse().unwrap(),
                item_name:record[1].to_string()});

        }
        Ok(())
    }

    pub fn get_name(&mut self, item_id:i32) -> String {
        for item in &self.items {
            if item.item_id == item_id {
                return item.item_name.clone();
            }
        }
        String::from("Root")

    }

}



mod tests {
    use super::*;
    #[test]
    fn test_load_csv() {
        let mut x = ItemNames::build("Test".to_string());
        let _: Result<(), Box<dyn Error>> = x.read_csv("./test_data/dimensions/cost_center_names.csv");

        assert_eq!(x.items.len(),40);
    }
}