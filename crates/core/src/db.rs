use parser::ParseError;

#[derive(Default)]
#[salsa::db(crate::Jar, parser::Jar)]
pub struct RootDatabase {
    storage: salsa::Storage<RootDatabase>,
}

impl RootDatabase {
    pub fn compile_string(&self, source: String) -> Result<parser::Program, ParseError> {
        let source = parser::ProgramSource::new(self, source);
        crate::compile::compile_program(self, source)
    }
}

impl salsa::Database for RootDatabase {}

impl salsa::ParallelDatabase for RootDatabase {
    fn snapshot(&self) -> salsa::Snapshot<RootDatabase> {
        salsa::Snapshot::new(RootDatabase {
            storage: self.storage.snapshot(),
        })
    }
}
