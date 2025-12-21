mod extract;
mod sep;

pub use self::extract::Extract;
pub use self::extract::extract;
pub use self::sep::SepOnce;

use crate::Error;
use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::regex::Regex;

pub trait Ctor<'a, C, O, H>: Regex<C> {
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error>;
}

pub trait Handler<C> {
    type Out;
    type Error: Into<Error>;

    fn invoke(&mut self, ctx: &C, span: &Span) -> Result<Self::Out, Self::Error>;
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for ()
where
    C: Context<'a, Orig<'a> = &'a str> + Match<'a>,
    H: Handler<C, Out = O>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        handler.invoke(ctx, &Span::default()).map_err(Into::into)
    }
}

impl<'a, C, O, H> Ctor<'a, C, O, H> for &str
where
    C: Context<'a, Orig<'a> = &'a str> + Match<'a>,
    H: Handler<C, Out = O>,
{
    fn construct(&self, ctx: &mut C, handler: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self)?;

        handler.invoke(ctx, &ret).map_err(Into::into)
    }
}

pub trait CtorOps<'a, C>: Sized
where
    C: Context<'a>,
{
    fn sep_once<S, R>(self, sep: S, right: R) -> SepOnce<C, Self, S, R>;
}

impl<'a, C, T: Regex<C>> CtorOps<'a, C> for T
where
    C: Context<'a>,
{
    fn sep_once<S, R>(self, sep: S, right: R) -> SepOnce<C, Self, S, R> {
        SepOnce::new(self, sep, right)
    }
}
