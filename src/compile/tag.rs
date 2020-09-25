use crate::Stmt;

#[derive(Debug, Clone)]
pub struct Tag {
    pub tag: Box<Stmt>,           // tag name
    pub id: Box<Stmt>,            // tag id
    pub classes: Vec<Stmt>,       // classes
    pub attrs: Vec<(Stmt, Stmt)>, // other attributes
    pub body: Vec<Stmt>,          // Bunch o' expressions
    pub closed: bool,             // <self-closing/> ?
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.tag == other.tag
            && self.id == other.id
            && self.classes == other.classes
            && self.body == other.body
            && self.closed == other.closed
            && self.attrs.iter().all(|(k, v)| {
                if let Some((_, o)) = other.attrs.iter().find(|(ok, _)| k == ok) {
                    o == v
                } else {
                    false
                }
            })
    }
}

impl Tag {
    pub fn new(tag: Stmt) -> Tag {
        Tag {
            tag: bx!(tag),
            id: bx!(Stmt::None),
            classes: vec![],
            attrs: vec![],
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

    pub fn set_id(&mut self, id: Stmt) {
        self.id = bx!(id);
    }

    pub fn set_body(&mut self, body: Vec<Stmt>) {
        self.body = body;
    }

    pub fn add_class(&mut self, class: Stmt) {
        self.classes.push(class);
    }

    pub fn add_attr(&mut self, name: Stmt, val: Stmt) {
        self.attrs.push((name, val));
    }
}
