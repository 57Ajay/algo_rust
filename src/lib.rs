pub mod data_structures {
    pub mod lists {
        pub mod vector;
    }
}

#[cfg(test)]
mod tests {
    use super::data_structures::lists::vector;
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
}
