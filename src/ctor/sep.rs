use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::Error;
use crate::ctor::Handler;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::regex::Regex;

#[derive(Copy)]
pub struct SepOnce<C, L, S, R> {
    left: L,
    sep: S,
    right: R,
    marker: PhantomData<C>,
}

impl<C, L, S, R> Clone for SepOnce<C, L, S, R>
where
    L: Clone,
    S: Clone,
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            sep: self.sep.clone(),
            right: self.right.clone(),
            marker: self.marker,
        }
    }
}

impl<C, L, S, R> SepOnce<C, L, S, R> {
    pub fn new(left: L, sep: S, right: R) -> Self {
        Self {
            left,
            sep,
            right,
            marker: PhantomData,
        }
    }
}

impl<'a, C, L, S, R, O1, O2, H> Ctor<'a, C, (O1, O2), H> for SepOnce<C, L, S, R>
where
    L: Ctor<'a, C, O1, H>,
    R: Ctor<'a, C, O2, H>,
    S: Regex<C>,
    C: Match<'a>,
    H: Handler<C>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<(O1, O2), Error> {
        let r = self.left.construct(ctx, func)?;
        let _ = ctx.try_mat(&self.sep)?;
        let l = self.right.construct(ctx, func)?;

        Ok((r, l))
    }
}

impl<'a, C, L, S, R> Regex<C> for SepOnce<C, L, S, R>
where
    S: Regex<C>,
    L: Regex<C>,
    R: Regex<C>,
    C: Match<'a>,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, Error> {
        let mut span = Span::new(ctx.offset(), 0);

        span.add_assign(ctx.try_mat(&self.left)?);
        span.add_assign(ctx.try_mat(&self.sep)?);
        span.add_assign(ctx.try_mat(&self.right)?);
        Ok(span)
    }
}
