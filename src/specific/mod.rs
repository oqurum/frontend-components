use serde::{Deserialize, Serialize};

mod id;
mod image;
mod isbn;
mod language;
mod source;

pub use id::*;
pub use image::*;
pub use isbn::*;
pub use language::*;
pub use source::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
