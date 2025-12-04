use std::{
    num::TryFromIntError,
    ops::{Index, IndexMut},
    str::FromStr,
};

use glam::IVec2;
use image::ImageBuffer;
use itertools::Itertools;
use tap::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    items: Vec<T>,
    size: IVec2,
}

impl<T> Grid<T> {
    pub fn from_positions(op: impl FnMut(IVec2) -> T, size: IVec2) -> Self {
        Self {
            items: Positions {
                current: IVec2::ZERO,
                size,
            }
            .map(op)
            .collect_vec(),
            size,
        }
    }
    pub fn from_value(value: T, size: IVec2) -> Self
    where
        T: Clone,
    {
        assert!(size.min_element() >= 0, "Height and width must be positive");
        Self {
            #[allow(clippy::cast_sign_loss)]
            items: std::iter::repeat_n(value, (size.x as usize) * (size.y as usize)).collect_vec(),
            size,
        }
    }

    pub fn from_row_major_ordered(values: impl IntoIterator<Item = T>, size: IVec2) -> Self {
        let items = values.into_iter().collect_vec();
        assert_eq!(
            items.len(),
            (size.x * size.y).try_conv::<usize>().expect("")
        );
        Self { items, size }
    }

    #[must_use]
    pub fn pixels<P>(&self, make_pixel: impl Fn(&T) -> P) -> ImageBuffer<P, Vec<P::Subpixel>>
    where
        P: image::Pixel,
    {
        ImageBuffer::from_fn(
            self.size.x.try_into().expect(""),
            self.size.y.try_into().expect(""),
            |x, y| {
                self.get(IVec2 {
                    x: x.try_into().expect(""),
                    y: y.try_into().expect(""),
                })
                .expect("Not to query outside of image")
                .pipe_ref(&make_pixel)
            },
        )
    }

    #[must_use]
    pub fn pixel_positions<P>(
        &self,
        make_pixel: impl Fn(IVec2, &T) -> P,
    ) -> ImageBuffer<P, Vec<P::Subpixel>>
    where
        P: image::Pixel,
    {
        ImageBuffer::from_fn(
            self.size.x.try_into().expect(""),
            self.size.y.try_into().expect(""),
            |x, y| {
                let pos = IVec2 {
                    x: x.try_into().expect(""),
                    y: y.try_into().expect(""),
                };
                make_pixel(pos, self.get(pos).expect("Not to query outside of image"))
            },
        )
    }

    #[must_use]
    pub const fn size(&self) -> IVec2 {
        self.size
    }
    #[must_use]
    pub fn get(&self, pos: IVec2) -> Option<&T> {
        self.index(pos).and_then(|idx| self.items.get(idx))
    }

    #[must_use]
    pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut T> {
        self.index(pos).and_then(|idx| self.items.get_mut(idx))
    }

    pub fn map<U>(self, op: impl FnMut(T) -> U) -> Grid<U> {
        Grid {
            size: self.size,
            items: self.items.into_iter().map(op).collect_vec(),
        }
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    #[must_use]
    pub const fn positions(&self) -> Positions {
        Positions {
            current: IVec2::ZERO,
            size: self.size,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    fn index(&self, pos: IVec2) -> Option<usize> {
        if pos.min_element() < 0 || pos.x >= self.size.x || pos.y >= self.size.y {
            None
        } else {
            Some((pos.y as usize) * (self.size.x as usize) + (pos.x as usize))
        }
    }
}

impl<T> Index<IVec2> for Grid<T> {
    type Output = T;

    fn index(&self, index: IVec2) -> &Self::Output {
        self.get(index).expect("Position out of bounds")
    }
}

impl<T> IndexMut<IVec2> for Grid<T> {
    fn index_mut(&mut self, index: IVec2) -> &mut Self::Output {
        self.get_mut(index).expect("Position out of bounds")
    }
}

impl FromStr for Grid<char> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (height, width) = (
            s.lines().count(),
            s.lines()
                .map(str::trim)
                .map(str::len)
                .unique()
                .at_most_one()
                .map_err(|_| Error::InconsistentLines)?
                .ok_or(Error::Empty)?,
        );
        Self {
            items: s.chars().filter(|c| !c.is_whitespace()).collect_vec(),
            size: IVec2 {
                x: width.try_into()?,
                y: height.try_into()?,
            },
        }
        .pipe(Ok)
    }
}

impl FromStr for Grid<u8> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (height, width) = (
            s.lines().count(),
            s.lines()
                .map(str::trim)
                .map(str::len)
                .unique()
                .at_most_one()
                .map_err(|_| Error::InconsistentLines)?
                .ok_or(Error::Empty)?,
        );
        if s.is_ascii() {
            Self {
                items: s
                    .as_bytes()
                    .iter()
                    .copied()
                    .filter(|b| !b.is_ascii_whitespace())
                    .collect_vec(),
                size: IVec2 {
                    x: width.try_into()?,
                    y: height.try_into()?,
                },
            }
            .pipe(Ok)
        } else {
            Err(Error::NonAscii)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Positions {
    current: IVec2,
    size: IVec2,
}

impl Iterator for Positions {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match (self.current, self.size) {
                (IVec2 { y, .. }, IVec2 { y: h, .. }) if y >= h => break None,
                (IVec2 { x, .. }, IVec2 { x: w, y: _ }) if x >= w => {
                    self.current.y += 1;
                    self.current.x = 0;
                }
                (current, _) => {
                    self.current.x += 1;
                    break Some(current);
                }
            }
        }
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, displaydoc::Display, thiserror::Error)]
pub enum Error {
    /// Not all provided lines were the same length
    InconsistentLines,
    /// The provided string had no lines
    Empty,
    /// Failed to convert between integer types
    IntegerConversion(#[from] TryFromIntError),
    /// Attempted to construct a byte grid from non-ascii text
    NonAscii,
}
