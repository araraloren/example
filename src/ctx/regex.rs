use std::str::CharIndices;

use super::Context;
use super::Regex;
use super::Span;

use crate::Error;
use crate::ctx::Match;

#[derive(Debug)]
pub struct RegexCtx<'a, T>
where
    T: ?Sized,
{
    dat: &'a T,
    offset: usize,
}

impl<T> Clone for RegexCtx<'_, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RegexCtx<'_, T> where T: ?Sized {}

impl<'a, T> RegexCtx<'a, T>
where
    T: ?Sized,
{
    pub fn new(dat: &'a T) -> Self {
        Self { dat, offset: 0 }
    }
}

impl<'a> Context<'a> for RegexCtx<'a, str> {
    type Orig<'b> = &'b str;

    type Item = char;

    type Iter<'b>
        = CharIndices<'b>
    where
        Self: 'b;

    fn len(&self) -> usize {
        self.dat.len()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn set_offset(&mut self, offset: usize) -> &mut Self {
        self.offset = offset;
        self
    }

    fn inc(&mut self, offset: usize) -> &mut Self {
        self.offset += offset;
        self
    }

    fn orig_at(&self, offset: usize) -> Result<Self::Orig<'a>, Error> {
        self.dat.get(offset..).ok_or(Error::OutOfBound)
    }

    fn peek_at(&self, offset: usize) -> Result<Self::Iter<'a>, Error> {
        Ok(self.orig_at(offset)?.char_indices())
    }

    fn orig_sub(&self, offset: usize, len: usize) -> Result<Self::Orig<'a>, Error> {
        self.dat
            .get(offset..(offset + len))
            .ok_or(Error::OutOfBound)
    }
}

impl<'a, T> Match<'a> for RegexCtx<'a, T>
where
    T: ?Sized,
    Self: Context<'a>,
{
    fn try_mat<Pat>(&mut self, pat: &Pat) -> Result<Span, Error>
    where
        Pat: Regex<RegexCtx<'a, T>> + ?Sized,
    {
        let ret = pat.try_parse(self)?;

        Ok(ret)
    }
}
