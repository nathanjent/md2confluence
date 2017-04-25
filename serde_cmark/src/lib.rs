extern crate pulldown_cmark;
#[macro_use]
extern crate serde;
#[cfg(test)]
#[macro_use]
extern crate serde_derive;

mod error;
mod ser;
mod de;

pub use error::{Error, Result};
pub use ser::{to_string, Serializer};
pub use de::{from_str, Deserializer};
