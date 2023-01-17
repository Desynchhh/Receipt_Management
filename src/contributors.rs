#[derive(Debug, PartialEq)]
pub struct Contributors {
    pub names: Vec<String>
}

impl Contributors {
    pub fn new(names:&str) -> Contributors {
        let mut vec:Vec<String> = Vec::new();

        for name in names.split(',') {
            vec.push(name.trim().to_owned());
        }

        Contributors { names: vec }
    }

    pub fn from_vector(names:Vec<String>) -> Contributors {
        Contributors { names }
    }


    pub fn to_string(&self) -> String {
        let mut string = String::from("[");
        for i in 0..self.names.len() {
            string.push_str(self.names[i].as_str());
            if i != self.names.len()-1 {
                string.push_str(", ");
            }
        }
        string.push(']');

        string
    }

    pub fn from_contributor_marking(marking:&str, receipt_contributors:&Contributors) -> Contributors {
        let mut vec:Vec<String> = Vec::new();

        for (i, char) in marking.to_lowercase().chars().enumerate() {
            if char == 'x' {
                vec.push(receipt_contributors.names[i].clone());
            }
        }

        Contributors { names: vec }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string_stringifies() {
        let contributors = Contributors { names: vec![String::from("Mikkel"), String::from("Thea")] };

        let expected_result = "[Mikkel, Thea]";
        let actual_result = contributors.to_string();

        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn create_new_contributors() {
        let expected_result = vec![String::from("Mikkel"), String::from("Thea")];
        let actual_result = Contributors::new("Mikkel, Thea");

        assert_eq!(expected_result, actual_result.names);
    }
}