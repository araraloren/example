use crate::Error;
use crate::ctor::Ctor;
use crate::ctx::Context;
use crate::ctx::Span;

use crate::ctor::Handler;
use crate::ctx::Match;
use crate::ctx::new_span_inc;

pub trait Regex<C> {
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error>;

    fn parse(&self, ctx: &mut C) -> bool {
        self.try_parse(ctx).is_ok()
    }
}

impl<C, F> Regex<C> for F
where
    F: Fn(&mut C) -> Result<Span, Error>,
{
    #[inline]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        (self)(ctx)
    }
}

impl<'a, C> Regex<C> for ()
where
    C: Context<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Ok(Span::new(ctx.offset(), 0))
    }
}

impl<'a, C> Regex<C> for &str
where
    C: Context<'a, Orig<'a> = &'a str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        Regex::try_parse(&LitString::new(self), ctx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LitString<'a> {
    val: &'a str,
}

impl<'a> LitString<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for LitString<'_>
where
    C: Match<'a, Orig<'a> = &'a str>,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        func.invoke(ctx, &ret).map_err(Into::into)
    }
}

impl<'a, C> Regex<C> for LitString<'_>
where
    C: Context<'a, Orig<'a> = &'a str>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::Error> {
        let mut ret = Err(Error::LitString);
        let literal_len = self.val.len();
        let remaining_len = ctx.len() - ctx.offset();

        if remaining_len >= literal_len && ctx.orig()?.starts_with(self.val) {
            ret = Ok(new_span_inc(ctx, literal_len));
        }
        ret
    }
}
