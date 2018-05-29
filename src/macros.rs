macro_rules! test_domain {
    ($domain:ident, $test_module:ident) => {
        #[cfg(test)]
        mod $test_module {
            use super::$domain;
            use std::iter::FromIterator;
            use traits::*;

            #[test]
            fn test_contains() {
                let domain = $domain::from_iter(vec![1, 3, 5, 7]);
                for i in 0..8 {
                    if i % 2 == 0 {
                        assert!(!domain.contains(&i));
                    } else {
                        assert!(domain.contains(&i));
                    }
                }
            }

            #[test]
            fn test_remove() {
                let mut domain = $domain::from_iter(vec![1, 2, 3, 4, 5]);
                assert_eq!(domain.size(), 5);

                assert!(domain.remove(&0).is_ok());
                assert_eq!(domain.size(), 5);
                assert!(domain.remove(&0).is_ok());
                assert_eq!(domain.size(), 5);

                assert!(domain.remove(&1).is_ok());
                assert_eq!(domain.size(), 4);
                assert!(domain.remove(&2).is_ok());
                assert_eq!(domain.size(), 3);
                assert!(domain.remove(&3).is_ok());
                assert_eq!(domain.size(), 2);
                assert!(domain.remove(&4).is_ok());
                assert_eq!(domain.size(), 1);
                assert!(domain.remove(&5).is_err());
                assert_eq!(domain.size(), 0);

                assert!(domain.remove(&0).is_ok());
                assert_eq!(domain.size(), 0);
                assert!(domain.remove(&0).is_ok());
                assert_eq!(domain.size(), 0);
            }

            #[test]
            fn test_assign() {
                let mut domain = $domain::from_iter(vec![1, 2, 3, 4, 5]);
                assert_eq!(domain.size(), 5);

                assert!(domain.assign(&4).is_ok());
                assert_eq!(domain.size(), 1);
                assert!(domain.assign(&4).is_ok());
                assert_eq!(domain.size(), 1);
                assert!(domain.assign(&2).is_err());
                assert_eq!(domain.size(), 0);
                assert!(domain.remove(&1).is_ok());
                assert_eq!(domain.size(), 0);
            }
        }
    };
}

macro_rules! test_bounded_domain {
    ($domain:ident, $test_module:ident) => {
        #[cfg(test)]
        mod $test_module {
            use super::$domain;
            use std::iter::FromIterator;
            use traits::*;

            #[test]
            fn test_adjust_min() {
                let mut domain = $domain::from_iter(vec![1, 2, 3, 4, 5]);
                assert_eq!(domain.min(), Some(&1));

                assert!(domain.adjust_min(&0).is_ok());
                assert_eq!(domain.min(), Some(&1));

                assert!(domain.adjust_min(&1).is_ok());
                assert_eq!(domain.min(), Some(&1));
                assert!(domain.adjust_min(&3).is_ok());
                assert_eq!(domain.min(), Some(&3));
                assert!(domain.adjust_min(&5).is_ok());
                assert_eq!(domain.min(), Some(&5));

                assert!(domain.adjust_min(&6).is_err());
                assert_eq!(domain.min(), None);
            }

            #[test]
            fn test_adjust_max() {
                let mut domain = $domain::from_iter(vec![1, 2, 3, 4, 5]);
                assert_eq!(domain.max(), Some(&5));

                assert!(domain.adjust_max(&6).is_ok());
                assert_eq!(domain.max(), Some(&5));

                assert!(domain.adjust_max(&5).is_ok());
                assert_eq!(domain.max(), Some(&5));
                assert!(domain.adjust_max(&3).is_ok());
                assert_eq!(domain.max(), Some(&3));
                assert!(domain.adjust_max(&1).is_ok());
                assert_eq!(domain.max(), Some(&1));

                assert!(domain.adjust_max(&0).is_err());
                assert_eq!(domain.max(), None);
            }
        }
    };
}
