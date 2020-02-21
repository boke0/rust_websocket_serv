use std::collections::HashMap;

enum State {
    Path,
    QueryKey,
    QueryVal,
    Fragment
}
pub struct Uri {
    pub path: String,
    pub query: HashMap<String, String>,
    pub fragment: String
}
impl Uri {
    pub fn from(uri: String) -> Uri {
        let mut path = String::new();
        let mut query = HashMap::new();
        let mut fragment = String::new();
        let mut key_buf = String::new();
        let mut val_buf = String::new();
        let mut state = State::Path;
        for c in uri.chars() {
            match c {
                '?' => state = State::QueryKey,
                '=' => state = State::QueryVal,
                '&' => {
                    state = State::QueryKey;
                    query.insert(key_buf, val_buf);
                    key_buf=String::new();
                    val_buf=String::new();
                },
                '#' => {
                    state = State::Fragment;
                    query.insert(key_buf, val_buf);
                    key_buf=String::new();
                    val_buf=String::new();
                },
                _ => match state {
                    State::Path => path.push(c),
                    State::QueryKey=> key_buf.push(c),
                    State::QueryVal => val_buf.push(c),
                    State::Fragment => fragment.push(c)
                }
            }
        }
        Uri {
            path: path,
            query: query,
            fragment: fragment
        }
    }
}
