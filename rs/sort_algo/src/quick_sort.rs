use crate::Sorter;
use std::borrow::{BorrowMut, Borrow};

pub struct Quicksort;

fn hoare_split<T: Ord>(slice: &mut [T], low: usize, high: usize) {
    let pivot = slice.get_mut(low + ((high - low) >> 1)).unwrap();
    let mut l = low;
    let mut r = high;
    loop {
        while slice[l] < pivot {
            l += 1;
        }
        while slice[r] > pivot {
            r -= 1;
        }
        if l >= r {
            break;
        }
        slice.swap(l, r);
    }
    hoare_split(slice, low, r);
    hoare_split(slice, r + 1, high)
}

impl Sorter for Quicksort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        if slice.is_empty() {
            return
        }
        hoare_split(slice, 0, slice.len() - 1);
    }
}

#[test]
fn test_quick_sort() {
    use super::*;

    let mut slice = vec![4, 1, 9, 2, 7, 3];
    Quicksort.sort(&mut slice);
    assert_eq!(slice, &[1, 2, 3, 4, 7, 9]);
}
