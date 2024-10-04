
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ValueKeys {
    pub keys: Vec<i32>,
    pub length: usize
}

impl std::fmt::Display for ValueKeys {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s= String::from("");
        let mut i = 0;        
        for d in &self.keys {
            if i > 0 {
                s.push_str(",");
            }
            s.push_str(&d.to_string());
            i = i +1;
        }

        write!(f, "[{}]", s)
    }
}
