use {
    crate::{parse, scan, Result, Stmt},
    std::{fs::File, io::Read, path::Path},
};

/// Compiled HTML template.
pub struct Template {
    source: String,
    compiled: Option<Vec<Stmt>>,
}

impl Template {
    pub fn new(source: String) -> Template {
        Template {
            source,
            compiled: None,
        }
    }

    pub fn stmts(&mut self) -> Result<&[Stmt]> {
        self.compile()?;
        if let Some(stmts) = &self.compiled {
            Ok(stmts)
        } else {
            Ok(&[])
        }
    }

    pub fn compile(&mut self) -> Result<()> {
        if self.compiled.is_none() {
            self.compiled = Some(
                scan(&self.source)
                    .and_then(|t| parse(&t))?
            );
        }
        Ok(())
    }
}

impl From<String> for Template {
    fn from(s: String) -> Template {
        Template::new(s)
    }
}

impl From<&str> for Template {
    fn from(s: &str) -> Template {
        Template::new(s.to_string())
    }
}

impl From<&Path> for Template {
    fn from(p: &Path) -> Template {
        File::open(p).unwrap().into()
    }
}

impl From<File> for Template {
    fn from(mut f: File) -> Template {
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        Template::new(s)
    }
}
