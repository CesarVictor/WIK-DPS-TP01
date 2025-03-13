use std::collections::HashMap;

pub(crate) struct Header {
    pub headers: HashMap<String, String>,
}

impl Header {
    pub fn new(headers: &[String]) -> Self {
        let mut header_map = HashMap::new();

        for line in headers {
            if let Some((key, value)) = line.split_once(": ") {
                header_map.insert(key.to_string(), value.to_string());
            }
        }

        Self {
            headers: header_map,
        }
    }

    pub fn serialize(&self) -> String {
        let mut json = String::from("{\"header\": {");
        let mut first = true;
        for (key, value) in &self.headers {
            if !first {
                json.push_str(", ");
            }
            first = false;
            let escaped_value = value.replace('"', "\\\"");
            json.push_str(&format!("\"{}\": \"{}\"", key, escaped_value));
        }
        json.push_str("}}");
        json
    }
}
