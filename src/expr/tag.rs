use {
    crate::Expr,
    std::{
        collections::HashMap,
        hash::{Hash, Hasher},
    },
};

#[derive(Debug, Clone, Eq)]
pub struct Tag {
    pub tag: Box<Expr>,             // tag name
    pub id: Box<Expr>,              // tag id
    pub classes: Vec<Expr>,         // classes
    pub attrs: HashMap<Expr, Expr>, // other attributes
    pub body: Vec<Expr>,            // Bunch o' expressions
    pub closed: bool,               // <self-closing/> ?
}

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
        self.id.hash(state);
        self.classes.hash(state);
        self.body.hash(state);
        self.closed.hash(state);
        for (k, v) in &self.attrs {
            k.hash(state);
            v.hash(state);
        }
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.tag == other.tag
            && self.id == other.id
            && self.classes == other.classes
            && self.body == other.body
            && self.closed == other.closed
            && self.attrs.iter().all(|(k, v)| {
                if let Some(o) = other.attrs.get(k) {
                    o == v
                } else {
                    false
                }
            })
    }
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

    pub fn add_attr(&mut self, name: Expr, val: Expr) {
        self.attrs.insert(name, val);
    }
}
