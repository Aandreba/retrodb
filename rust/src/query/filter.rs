use std::fmt::Write;

pub fn and(lhs: impl QueryFilter, rhs: impl QueryFilter) -> impl QueryFilter {
    struct And<L, R>(L, R);
    impl<L: QueryFilter, R: QueryFilter> QueryFilter for And<L, R> {
        #[inline]
        fn build(&self, curr: &mut u32, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_char('(')?;
            self.0.build(curr, f)?;
            f.write_str(") AND (")?;
            self.1.build(curr, f)?;
            f.write_char(')')
        }
    }

    return And(lhs, rhs);
}

pub fn or(lhs: impl QueryFilter, rhs: impl QueryFilter) -> impl QueryFilter {
    struct Or<L, R>(L, R);
    impl<L: QueryFilter, R: QueryFilter> QueryFilter for Or<L, R> {
        #[inline]
        fn build(&self, curr: &mut u32, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_char('(')?;
            self.0.build(curr, f)?;
            f.write_str(") OR (")?;
            self.1.build(curr, f)?;
            f.write_char(')')
        }
    }

    return Or(lhs, rhs);
}

pub fn not(lhs: impl QueryFilter) -> impl QueryFilter {
    #[repr(transparent)]
    struct Not<T>(T);
    impl<T: QueryFilter> QueryFilter for Not<T> {
        #[inline]
        fn build(&self, curr: &mut u32, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("NOT (")?;
            self.0.build(curr, f)?;
            f.write_char(')')
        }
    }

    return Not(lhs);
}

pub trait QueryFilter {
    fn build(&self, curr: &mut u32, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

pub trait QueryValue<T> {
    fn build(&self, curr: &mut u32, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SerialId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Developer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Publisher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Rating;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Users;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Franchise;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ReleaseYear;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ReleaseMonth;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Region;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Genre;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DisplayName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FullName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Platform;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Roms;
