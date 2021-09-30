//! This module consists of different utilities, that are being used across velox.

/// Describes type of content that will be displayed on a webview window
#[derive(Debug, Clone)]
pub enum ContentType {
    Url(String),
    File(String),
}

impl ContentType {
    pub fn get_content(&self) -> &str {
        match self {
            ContentType::File(content) => content,
            ContentType::Url(content) => content,
        }
    }
}

pub fn is_dev() -> bool {
    cfg!(debug_assertions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        // check if function is_dev returns true on dev env
        if cfg!(debug_assertions) {
            assert!(is_dev())
        } else {
            assert!(is_dev());
        }
    }
}
