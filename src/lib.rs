pub mod ctor;
pub mod ctx;
pub mod neu;
pub mod regex;

pub trait MayDebug {}

impl<T> MayDebug for T {}

pub mod prelude {
    pub use crate::ctor;
    pub use crate::ctor::CtorOps;
    pub use crate::ctx::CharsCtx;
    pub use crate::ctx::Context;
    pub use crate::ctx::Match;
    pub use crate::ctx::MatchExt;
    pub use crate::ctx::RegexCtx;
    pub use crate::ctx::Span;
    pub use crate::neu;
    pub use crate::neu::Neu;
    pub use crate::neu::NeuIntoRegexOps;
    pub use crate::regex;
    pub use crate::regex::Regex;
}

use std::fmt::Display;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    LitString,

    Between,

    OutOfBound,

    Uid(usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}
