//! A Symbol is an interned String, created and managed by
//! [`Interner`] and stored in CACHE.

use {
    crate::Interner,
    std::{
        fmt, ops,
        sync::{Once, RwLock},
    },
};

/// Our String cache, which our Symbols reference.
static mut CACHE: Option<RwLock<Interner>> = None;

/// Only setup the CACHE once.
static SETUP: Once = Once::new();

#[derive(Clone, Ord, PartialOrd, Eq)]
pub struct Symbol(u32);

fn cache() -> Option<&'static RwLock<Interner>> {
    unsafe {
        SETUP.call_once(|| CACHE = Some(RwLock::new(Interner::new())));
        CACHE.as_ref()
    }
}

impl Symbol {
    pub fn as_ref(&self) -> &str {
        self.to_str()
    }

    pub fn empty() -> Symbol {
        Symbol(0)
    }

    pub fn to_str(&self) -> &str {
        if let Some(cache) = cache() {
            cache.read().unwrap().lookup(self.0)
        } else {
            ""
        }
    }
    pub fn from<S: AsRef<str>>(s: S) -> Symbol {
        if let Some(cache) = cache() {
            Symbol(cache.write().unwrap().intern(s.as_ref()))
        } else {
            Symbol::empty()
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol({}, {})", self.0, self.to_str())
    }
}

impl ops::Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.to_str()
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.to_str() == other.to_str()
    }
}

impl PartialEq<&str> for Symbol {
    fn eq(&self, other: &&str) -> bool {
        self.to_str() == *other
    }
}

impl PartialEq<String> for Symbol {
    fn eq(&self, other: &String) -> bool {
        self.to_str() == other
    }
}

impl From<String> for Symbol {
    fn from(s: String) -> Symbol {
        Symbol::from(s)
    }
}

impl From<&String> for Symbol {
    fn from(s: &String) -> Symbol {
        Symbol::from(s.as_str())
    }
}

impl From<&str> for Symbol {
    fn from(s: &str) -> Symbol {
        Symbol::from(s)
    }
}

impl From<&Symbol> for Symbol {
    fn from(s: &Symbol) -> Symbol {
        s.clone()
    }
}
