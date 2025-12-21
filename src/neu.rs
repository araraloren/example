mod times;

use crate::MayDebug;
use crate::ctx::Context;

pub use self::times::Between;

pub trait Neu<T> {
    fn is_match(&self, other: &T) -> bool;

    fn min_length(&self) -> usize {
        1
    }
}

impl<T> Neu<T> for char
where
    T: MayDebug,
    Self: PartialEq<T>,
{
    #[inline(always)]
    fn is_match(&self, other: &T) -> bool {
        self == other
    }

    fn min_length(&self) -> usize {
        self.len_utf8()
    }
}

// beg: None => match nothing
// end: None => match all
#[inline(always)]
pub(crate) fn calc_length(beg: Option<usize>, end: Option<usize>, remaining_len: usize) -> usize {
    beg.map(|v| end.unwrap_or(remaining_len) - v)
        .unwrap_or_default()
}

pub trait NeuIntoRegexOps<'a, C>
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn between<const M: usize, const N: usize>(self) -> Between<M, N, C, Self>;

    fn at_least<const M: usize>(self) -> Between<M, { usize::MAX }, C, Self> {
        self.between::<M, { usize::MAX }>()
    }

    fn at_most<const N: usize>(self) -> Between<0, N, C, Self> {
        self.between::<0, N>()
    }
}

impl<'a, C, U> NeuIntoRegexOps<'a, C> for U
where
    C: Context<'a>,
    Self: Sized + Neu<C::Item>,
{
    fn between<const M: usize, const N: usize>(self) -> Between<M, N, C, Self> {
        Between::new(self)
    }
}
