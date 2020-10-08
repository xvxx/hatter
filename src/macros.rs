macro_rules! bx {
    ($ex:expr) => {
        Box::new($ex)
    };
}

macro_rules! rc {
    ($ex:expr) => {
        Rc::new($ex)
    };
}

macro_rules! cell {
    ($ex:expr) => {
        RefCell::new($ex)
    };
}

macro_rules! rcell {
    ($ex:expr) => {
        rc!(cell!($ex))
    };
}
