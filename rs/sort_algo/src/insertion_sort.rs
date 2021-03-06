use crate::Sorter;

pub struct Insertionsort {
    pub smart: bool,
}

impl Sorter for Insertionsort {
    fn sort<T: Ord>(&self, slice: &mut [T]) {
        for unsorted in 1..slice.len() {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // 使用二分搜索来找到插入位置
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1);
            }
        }
    }
}

#[test]
fn test_insert_sort() {
    use super::*;

    let mut slice = vec![4, 1, 9, 2, 7, 3];
    Insertionsort { smart: false }.sort(&mut slice);
    assert_eq!(slice, &[1, 2, 3, 4, 7, 9]);

    let mut slice = vec![4, 1, 9, 2, 7, 3];
    Insertionsort { smart: true }.sort(&mut slice);
    assert_eq!(slice, &[1, 2, 3, 4, 7, 9]);
}
