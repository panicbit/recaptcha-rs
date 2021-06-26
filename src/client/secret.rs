use std::fmt;

pub(crate) struct Secret(String);

impl Secret {
    pub fn new(secret: String) -> Self {
        Self(secret)
    }

    pub fn reveal(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Secret")
            .field(&"<redacted>")
            .finish()
    }
}
