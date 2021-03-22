#[derive(Debug, Clone)]
pub enum Object {
    String(String),
}

impl Into<Object> for &str {
    fn into(self) -> Object {
        Object::String(self.to_string()) // TODO Object(String) should be Object(&str)
    }
}

impl Into<Object> for String {
    fn into(self) -> Object {
        Object::String(self)
    }
}