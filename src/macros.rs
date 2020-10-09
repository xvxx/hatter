/// Box an object.
macro_rules! bx {
    ($ex:expr) => {
        Box::new($ex)
    };
}

/// Rc an object.
macro_rules! rc {
    ($ex:expr) => {
        Rc::new($ex)
    };
}

/// RefCell an object.
macro_rules! cell {
    ($ex:expr) => {
        RefCell::new($ex)
    };
}

/// Rc::new(RefCell::new(X))
macro_rules! rcell {
    ($ex:expr) => {
        rc!(cell!($ex))
    };
}
