//! API for velox that can be used to interact with native window or filesystem.
//! These API can also be called from javascript using "__VELOX__" object.

pub mod fs;
pub mod notification;
pub mod subprocess;
pub mod window;
