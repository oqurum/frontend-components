use serde::{Serialize, Deserialize};

mod language;
mod image;
mod source;
mod isbn;
mod id;

pub use language::*;
pub use image::*;
pub use source::*;
pub use isbn::*;
pub use id::*;




#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}