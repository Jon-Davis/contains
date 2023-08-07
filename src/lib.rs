//! # Contains
//! The Contains crate has 2 traits Container and In.
//!
//! ## Container
//! The Container trait can be used to abstract over
//! types that can contain items: `Vec<T>`, `&[T]`, `HashMap<T>`, `Option<T>`, ect.
//! ```rust
//! use contains::Container;
//!
//! let vec = vec![1, 2, 3, 4, 5];
//! let range = 0..5;
//! let option = Some(3);
//!
//! let containers: &[&dyn Container<usize>] = &[&vec, &range, &option];
//! for container in containers {
//!   assert!(container.does_contain(&3));
//! }
//! ```
//!
//! ## In
//! The In trait is the Inverse of the Container trait and represents a type that is in
//! a container. Mainly it reverse the call order by providing the `is_in` method.
//! ```rust
//! use contains::{Container, In};
//!
//! let range = 0..5;
//! assert!(range.does_contain(&3));    // using does_contain
//! assert!(3.is_in(&range));           // using in
//! ```

/// ## Container
/// The Container trait can be used to abstract over
/// types that can contain items: `Vec<T>`, `&[T]`, `HashMap<T>`, `Option<T>`, ect.
/// ```rust
/// use contains::Container;
///
/// let vec = vec![1, 2, 3, 4, 5];
/// let range = 0..5;
/// let option = Some(3);
///
/// let containers: &[&dyn Container<usize>] = &[&vec, &range, &option];
/// for container in containers {
///   assert!(container.does_contain(&3));
/// }
/// ```
///
/// A `&str` is considered a container of `&str` and `&[T]` is considered a container of `&[T]`.
/// ```rust
/// use contains::Container;
///
/// assert!("hello world!".does_contain(&"hello"));
///
/// assert!([1,2,3,4,5].does_contain(&[3, 4]));
/// ```
pub trait Container<T> {
    fn does_contain(&self, item: &T) -> bool;
}

/// ## In
/// The In trait is the Inverse of the Container trait and represents a type that is in
/// a container. Mainly it reverse the call order by providing the `is_in` method.
/// ```rust
/// use contains::{Container, In};
///
/// let range = 0..5;
/// assert!(range.does_contain(&3));    // using does_contain
/// assert!(3.is_in(&range));           // using in
/// ```
pub trait In<C> {
    fn is_in(&self, container: &C) -> bool;
}

impl<C, T> In<C> for T
where
    C: Container<T>,
{
    fn is_in(&self, container: &C) -> bool {
        container.does_contain(self)
    }
}

impl<T> Container<T> for Vec<T>
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for Option<T>
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        matches!(self, Some(x) if x == item)
    }
}

impl<T, U> Container<T> for Result<T, U>
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        matches!(self, Ok(x) if x == item)
    }
}

impl<T> Container<T> for std::collections::HashSet<T>
where
    T: Eq + std::hash::Hash,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for std::collections::BTreeSet<T>
where
    T: Ord,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for std::collections::LinkedList<T>
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for std::collections::VecDeque<T>
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl Container<&str> for &str {
    fn does_contain(&self, item: &&str) -> bool {
        self.contains(item)
    }
}

impl Container<&str> for String {
    fn does_contain(&self, item: &&str) -> bool {
        self.contains(item)
    }
}

impl<T, const N: usize> Container<T> for [T; N]
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for &[T]
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<&[T]> for &[T]
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &&[T]) -> bool {
        self.windows(item.len()).any(|slice| &slice == item)
    }
}

impl<T, const N1: usize, const N: usize> Container<[T; N1]> for [T; N]
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &[T; N1]) -> bool {
        let container: &[T] = self;
        let item: &[T] = item;
        container.does_contain(&item)
    }
}

impl<T, const N1: usize> Container<[T; N1]> for &[T]
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &[T; N1]) -> bool {
        let container: &[T] = self;
        let item: &[T] = item;
        container.does_contain(&item)
    }
}

impl<T, const N: usize> Container<&[T]> for [T; N]
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &&[T]) -> bool {
        let container: &[T] = self;
        let item: &[T] = item;
        container.does_contain(&item)
    }
}

impl<T, const N1: usize> Container<[T; N1]> for Vec<T>
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &[T; N1]) -> bool {
        let container: &[T] = self;
        let item: &[T] = item;
        container.does_contain(&item)
    }
}

impl<T> Container<&[T]> for Vec<T>
where
    T: PartialEq<T>,
{
    fn does_contain(&self, item: &&[T]) -> bool {
        let container: &[T] = self;
        let item: &[T] = item;
        container.does_contain(&item)
    }
}

impl<T> Container<T> for std::ops::Range<T>
where
    T: PartialOrd<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for std::ops::RangeFrom<T>
where
    T: PartialOrd<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for std::ops::RangeTo<T>
where
    T: PartialOrd<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for std::ops::RangeFull
where
    T: PartialOrd<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        std::ops::RangeBounds::contains(self, item)
    }
}

impl<T> Container<T> for std::ops::RangeInclusive<T>
where
    T: PartialOrd<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

impl<T> Container<T> for std::ops::RangeToInclusive<T>
where
    T: PartialOrd<T>,
{
    fn does_contain(&self, item: &T) -> bool {
        self.contains(item)
    }
}

#[test]
fn test_container() {
    let array = [1, 2, 3, 4, 5];
    let slice = &[1, 2, 3, 4, 5] as &[i32];
    let vec = vec![1, 2, 3, 4, 5];
    let opt = Some(3);
    let res: Result<i32, i32> = Ok(3);
    let bts = {
        let mut init = std::collections::BTreeSet::new();
        init.insert(3);
        init
    };
    let hs = {
        let mut init = std::collections::HashSet::new();
        init.insert(3);
        init
    };
    let rng = 0..6;

    let collections: &[&dyn Container<i32>] = &[&array, &slice, &vec, &opt, &res, &bts, &hs, &rng];
    for container in collections {
        assert!(container.does_contain(&3));
    }

    let str = "Hello World";
    let string = String::from("Hello World");
    let haystacks: &[&dyn Container<&str>] = &[&str, &string];
    for container in haystacks {
        assert!(container.does_contain(&"Hello"));
    }
}
