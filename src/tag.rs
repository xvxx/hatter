use {crate::Stmt, std::collections::HashMap};

#[derive(Debug)]
pub struct Tag {
    pub tag: String,                    // tag name
    pub classes: Vec<String>,           // classes
    pub attrs: HashMap<String, String>, // other attributes
    pub contents: Vec<Stmt>,            // Stmt::Block
    closed: bool,                       // <self-closing/> ?
}

impl Tag {
    pub fn new(tag: String) -> Tag {
        Tag {
            tag,
            classes: vec![],
            attrs: HashMap::new(),
            closed: false,
            contents: vec![],
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn add_class<S: AsRef<str>>(&mut self, class: S) {
        self.classes.push(class.as_ref().to_string())
    }

    pub fn add_attr<S: AsRef<str>, X: AsRef<str>>(&mut self, name: S, val: X) {
        self.attrs
            .insert(name.as_ref().to_string(), val.as_ref().to_string());
    }
}
