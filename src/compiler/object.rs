#[derive(Debug, Clone)]
pub enum Object {
    String(String),
}

impl Into<Object> for String {
    fn into(self) -> Object {
        Object::String(self)
    }
}