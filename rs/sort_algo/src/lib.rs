mod bubble_sort;
mod insertion_sort;
mod selection_sort;
mod quick_sort;

pub trait Sorter {
    fn sort<T: Ord>(&self, slice: &mut [T]);
}

struct StdSorter;
impl Sorter for StdSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut slice = vec![4, 3, 2, 1];
        StdSorter.sort(&mut slice);
        assert_eq!(slice, &[1, 2, 3, 4])
    }
}
