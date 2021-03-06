use crate::Sorter;

pub struct Selectionsort;

impl Sorter for Selectionsort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        for unsorted in 0..slice.len() {
            let smallest_item_idx = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice is empty");
            if unsorted != smallest_item_idx {
                slice.swap(unsorted, smallest_item_idx);
            }
        }
    }
}

#[test]
fn test_selection_sort() {
    use super::*;

    let mut slice = vec![4, 1, 9, 2, 7, 3];
    Selectionsort.sort(&mut slice);
    assert_eq!(slice, &[1, 2, 3, 4, 7, 9]);
}
