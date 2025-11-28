pub mod data_structures {
    pub mod lists {
        pub mod queue;
        pub mod singly_linked_list;
        pub mod vector;
    }
    pub mod searching;
}

#[cfg(test)]
mod tests {
    use super::data_structures::lists::{singly_linked_list, vector};
    use super::data_structures::searching::BinarySearchExt;
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
}
