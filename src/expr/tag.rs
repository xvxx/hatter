use {crate::Expr, std::collections::HashMap};

#[derive(Debug, Clone)]
pub struct Tag {
    tag: Box<Expr>,               // tag name
    id: Box<Expr>,                // tag id
    classes: Vec<Expr>,           // classes
    attrs: HashMap<String, Expr>, // other attributes
    body: Vec<Expr>,              // Bunch o' expressions
    closed: bool,                 // <self-closing/> ?
}

impl Tag {
    pub fn new(tag: Expr) -> Tag {
        Tag {
            tag: bx!(tag),
            id: bx!(Expr::None),
            classes: vec![],
            attrs: HashMap::new(),
            closed: false,
            body: vec![],
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn set_id(&mut self, id: Expr) {
        self.id = bx!(id);
    }

    pub fn set_body(&mut self, body: Vec<Expr>) {
        self.body = body;
    }

    pub fn add_class(&mut self, class: Expr) {
        self.classes.push(class);
    }

    pub fn add_attr<S: AsRef<str>>(&mut self, name: S, val: Expr) {
        self.attrs.insert(name.as_ref().to_string(), val);
    }
}
