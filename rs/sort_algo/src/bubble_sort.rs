use crate::Sorter;

#[allow(dead_code)]
pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn test_bubble_sort() {
    use super::*;

    let mut slice = vec![4, 1, 9, 2, 7, 3];
    Bubblesort.sort(&mut slice);
    assert_eq!(slice, &[1, 2, 3, 4, 7, 9])
}
