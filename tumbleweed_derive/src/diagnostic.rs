// Adapted from: diesel/diesel_derives/src/diagnostic_shim.rs

#[cfg(feature = "nightly")]
pub use proc_macro::Diagnostic;

#[cfg(not(feature = "nightly"))]
pub struct Diagnostic {
    message: String,
    level: Level,
}

#[cfg(not(feature = "nightly"))]
impl Diagnostic {
    pub fn error<T: Into<String>>(msg: T) -> Self {
        Diagnostic {
            message: msg.into(),
            level: Level::Error,
        }
    }

    #[allow(dead_code)]
    pub fn warning<T: Into<String>>(msg: T) -> Self {
        Diagnostic {
            message: msg.into(),
            level: Level::Warning,
        }
    }

    #[allow(dead_code)]
    pub fn help<T: Into<String>>(mut self, msg: T) -> Self {
        self.message.push_str("\n");
        self.message.push_str(&msg.into());
        self
    }

    #[allow(dead_code)]
    pub fn note(self, msg: &str) -> Self {
        self.help(msg)
    }

    pub fn emit(self) {
        match self.level {
            Level::Error => panic!("{}", self.message),
            Level::Warning => println!("{}", self.message),
        }
    }
}

#[cfg(not(feature = "nightly"))]
#[allow(dead_code)]
enum Level {
    Warning,
    Error,
}
