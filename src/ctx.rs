mod regex;
mod span;

use crate::Error;
use crate::MayDebug;
use crate::ctor::Ctor;
use crate::ctor::Extract;
use crate::ctor::Handler;
use crate::ctor::extract;
use crate::regex::Regex;

pub use self::regex::RegexCtx;
pub use self::span::Span;

pub type CharsCtx<'a> = RegexCtx<'a, str>;

pub trait Context<'a> {
    type Orig<'b>;

    type Item: MayDebug;

    type Iter<'b>: Iterator<Item = (usize, Self::Item)>
    where
        Self: 'b;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn offset(&self) -> usize;

    fn set_offset(&mut self, offset: usize) -> &mut Self;

    fn inc(&mut self, offset: usize) -> &mut Self;

    fn peek(&self) -> Result<Self::Iter<'a>, Error> {
        self.peek_at(self.offset())
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error>;

    fn orig(&self) -> Result<Self::Orig<'a>, Error> {
        self.orig_at(self.offset())
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error>;

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error>;
}

pub trait Match<'a>: Context<'a>
where
    Self: Sized,
{
    fn try_mat<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<Self> + ?Sized;
}

pub trait MatchExt<'a>
where
    Self: Context<'a> + Sized,
{
    fn ctor<P, O>(&mut self, pat: &P) -> Result<O, Error>
    where
        P: Ctor<'a, Self, O, Extract<Self::Orig<'a>>>,
        Extract<Self::Orig<'a>>: Handler<Self>,
    {
        pat.construct(self, &mut extract())
    }
}

impl<'a, C> MatchExt<'a> for C where C: Sized + Match<'a> {}

// make new span [offset, offset + len) and increment offset
pub(crate) fn new_span_inc<'a>(ctx: &mut impl Context<'a>, len: usize) -> Span {
    let span = Span::new(ctx.offset(), len);

    ctx.inc(len);
    span
}
