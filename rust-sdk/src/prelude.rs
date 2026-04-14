//! Convenience re-exports for plugin authors.
//!
//! ```rust
//! use forgeide_plugin::prelude::*;
//! ```

pub use crate::types::{PluginContext, PluginInput, PluginOutput};
pub use crate::host::{read_file, kb_search};
pub use crate::capabilities;
