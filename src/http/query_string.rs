use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct QueryString {
    data: BTreeMap<String, Value>,
}

#[derive(Clone, Debug)]
pub enum Value {
    Single(String),
    Multiple(Vec<String>),
}

impl QueryString {
    pub fn get(&self, key: String) -> Option<&Value> {
        self.data.get(&key)
    }
}

impl From<&str> for QueryString {
    fn from(s: &str) -> Self {
        let mut data = BTreeMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key.to_string())
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![prev_val.to_string(), val.to_string()]);
                    },
                    Value::Multiple(vec) => vec.push(val.to_string()),
                })
                .or_insert(Value::Single(val.to_string()));
        }

        QueryString { data }
    }
}
