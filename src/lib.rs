//! # Rost
//!
//! A library for rosting. --- Ahem, I meant sorting.
//!
//! This crate is inspired by <https://github.com/FedericoStra/Sorting>.

/// An algorithm for sorting.
///
/// This trait identifies algorithms suitable for sorting slices.
pub trait SortingAlgorithm {
    /// Sorts the given slice.
    fn sort<T: Ord>(&self, slice: &mut [T]);
}

/// <https://github.com/orlp/pdqsort>
pub struct PDQsort;

impl SortingAlgorithm for PDQsort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        slice.sort_unstable();
    }
}

/// <https://en.wikipedia.org/wiki/Timsort>
pub struct Timsort;

impl SortingAlgorithm for Timsort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        slice.sort();
    }
}

/// Sorts the given slice using a specific [`SortingAlgorithm`].
pub fn sort<T: Ord, A: SortingAlgorithm>(slice: &mut [T], algo: &A) {
    algo.sort(slice);
}

/// Checks if the elements of the slice are sorted.
///
/// That is, for each element `a` and its following element `b`, `a <= b` must hold. If the
/// slice contains exactly zero or one element, `true` is returned.
///
/// Note that if `T` is only `PartialOrd`, but not `Ord`, the above definition implies
/// that this function returns `false` if any two consecutive items are not comparable.
pub fn is_sorted<T: PartialOrd>(slice: &[T]) -> bool {
    for i in 1..slice.len() {
        if let Some(std::cmp::Ordering::Greater) | None = slice[i - 1].partial_cmp(&slice[i]) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timsort() {
        let mut v = [1, 2, 3, 6, 3, 1, 3, -4, 6, -2, 3, 0, 5, 3, 0, 12];
        Timsort.sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn pdqsort() {
        let mut v = [1, 2, 3, 6, 3, 1, 3, -4, 6, -2, 3, 0, 5, 3, 0, 12];
        PDQsort.sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn it_sorts() {
        let mut v = [1, 2, 3, 6, 3, 1, 3, -4, 6, -2, 3, 0, 5, 3, 0, 12];
        sort(&mut v, &Timsort);
        assert!(is_sorted(&v));
    }
}
