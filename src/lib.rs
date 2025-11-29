pub mod data_structures {
    pub mod lists {
        pub mod queue;
        pub mod singly_linked_list;
        pub mod vector;
    }
    pub mod searching;
    pub mod sorting {
        pub mod merge_sort;
    }
}

#[cfg(test)]
mod tests {
    use super::data_structures::lists::{singly_linked_list, vector};
    use super::data_structures::searching::BinarySearchExt;
    use super::data_structures::sorting::merge_sort::merge_sort;
    #[test]
    fn test_push_and_indexing() {
        let mut v = vector::MyVec::new();
        v.push(10);
        v.push(20);
        v.push(30);

        assert_eq!(v[0], 10);
        assert_eq!(v[1], 20);
        assert_eq!(v[2], 30);
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_capacity_growth() {
        let mut v = vector::MyVec::new();
        for i in 0..100 {
            v.push(i);
        }
        assert_eq!(v.len(), 100);
        assert_eq!(v[99], 99);
        assert!(v.cap >= 100);
    }

    #[test]
    fn test_iterators() {
        let mut v = vector::MyVec::new();
        v.push(1);
        v.push(2);

        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_drop() {
        let mut v = vector::MyVec::new();
        v.push(String::from("Hello"));
        v.push(String::from("World"));
    }

    #[test]
    fn test_list_iter() {
        let mut list = singly_linked_list::LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_binary_search() {
        let arr = [1, 2, 2, 2, 4, 5];

        assert_eq!(arr.lower_bound(&2), 1);

        assert_eq!(arr.upper_bound(&2), 4);

        assert_eq!(arr.lower_bound(&3), 4);
        assert_eq!(arr.upper_bound(&3), 4);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_random_small() {
        let mut arr = vec![10, -2, 33, 0, 5, 4, 4, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![-2, 0, 1, 4, 4, 5, 10, 33]);
    }

    #[test]
    fn test_random_large() {
        let rng: i32 = rand::random_range(-1_000_000..100_000);

        let mut arr: Vec<i32> = (0..10_000).map(|_| rng).collect();

        let mut expected = arr.clone();

        merge_sort(&mut arr);
        expected.sort();

        assert_eq!(arr, expected);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 1, 3, 3, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 3, 5]);
    }
}
