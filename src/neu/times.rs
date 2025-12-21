use std::marker::PhantomData;

use crate::ctor::Ctor;

use crate::ctor::Handler;
use crate::ctx::Context;
use crate::ctx::Match;
use crate::ctx::Span;
use crate::ctx::new_span_inc;
use crate::Error;
use crate::neu::calc_length;
use crate::regex::Regex;

use super::Neu;

#[derive(Copy)]
pub struct Between<const M: usize, const N: usize, C, U> {
    unit: U,

    marker: PhantomData<C>,
}

impl<const M: usize, const N: usize, C, U> Clone for Between<M, N, C, U>
where
    U: Clone,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),

            marker: self.marker,
        }
    }
}

impl<const M: usize, const N: usize, C, U> Between<M, N, C, U> {
    pub fn new(unit: U) -> Self {
        Self {
            unit,

            marker: PhantomData,
        }
    }
}

impl<'a, const M: usize, const N: usize, U, C, O, H> Ctor<'a, C, O, H> for Between<M, N, C, U>
where
    U: Neu<C::Item>,
    C: Match<'a> + 'a,
    H: Handler<C, Out = O>,
{
    #[inline(always)]
    fn construct(&self, ctx: &mut C, func: &mut H) -> Result<O, Error> {
        let ret = ctx.try_mat(self);

        func.invoke(ctx, &ret?).map_err(Into::into)
    }
}

impl<'a, const M: usize, const N: usize, U, C> Regex<C> for Between<M, N, C, U>
where
    U: Neu<C::Item>,
    C: Context<'a> + 'a,
{
    #[inline(always)]
    fn try_parse(&self, ctx: &mut C) -> Result<Span, crate::Error> {
        let mut cnt = 0;
        let mut beg = None;
        let mut end = None;
        let mut ret = Err(Error::Between);
        let mut iter = ctx.peek()?;
        let remaining_len = ctx.len() - ctx.offset();

        if remaining_len >= M * self.unit.min_length() {
            while cnt < N {
                if let Some(pair) = iter.next() {
                    if self.unit.is_match(&pair.1) {
                        cnt += 1;
                        if beg.is_none() {
                            beg = Some(pair.0);
                        }
                        continue;
                    } else {
                        end = Some(pair);
                    }
                }
                break;
            }
            if cnt >= M {
                let end = end.or_else(|| iter.next());
                let len = calc_length(beg, end.map(|v| v.0), remaining_len);

                ret = Ok(new_span_inc(ctx, len));
            }
        }
        ret
    }
}
