// Test cases for the functions in the crate

#[cfg(test)]
mod filter_test {
    use crate::functions::filter;

    #[test]
    fn test_filter_map() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = filter(arr, |x| {
            match x % 2 {
                0 => Some(x * 2),
                _ => None,
            }
        });

        assert_eq!(result, vec![4, 8]);
    }
}

#[cfg(test)]
mod for_each_family_test {
    use crate::functions::{for_each, for_each_mutable};

    #[test]
    fn for_each_basic_test() {
        let arr = vec![1, 2, 3, 4, 5];
        for_each(&arr, |x| println!("{}", x));
    }

    #[test]
    fn for_each_mutable_integer_test() {
        let mut result = Vec::new();
        let arr = vec![1, 2, 3, 4, 5];

        for_each_mutable(&arr, |x| result.push(x * 2));

        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}
#[cfg(test)]
mod map_tests {
    use crate::functions::map;

    #[test]
    fn add_one() {
        let arr = vec![1, 2, 3, 4, 5];
        
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        let incremented = map(arr, add_one);
        assert_eq!(incremented, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn add_one_with_closure() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = map(arr, |x| x + 1);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn take_key_value_on_struct() {
        #[derive(Debug, PartialEq, Clone)]
        struct TestArray {
            key: i32,
            value: i32,
        }

        let arr = vec![
            TestArray { key: 1, value: 100 },
            TestArray { key: 2, value: 200 },
            TestArray { key: 3, value: 300 },
            TestArray { key: 4, value: 450 },
            TestArray { key: 5, value: 50000 },
        ];

        let result = map(arr, |x| (x.value, x.key));
        assert_eq!(
            result, 
            vec![(100, 1), (200, 2), (300, 3), (450, 4), (50000, 5)],
        );

        let arr = vec![
            TestArray { key: 1, value: 100 },
            TestArray { key: 2, value: 200 },
            TestArray { key: 3, value: 300 },
            TestArray { key: 4, value: 450 },
            TestArray { key: 5, value: 50000 },
        ];

        let result = map(arr, |x| (x.key, x.value + 10));
        assert_eq!(
            result, 
            vec![(1, 110), (2, 210), (3, 310), (4, 460), (5, 50010)],
        );
    }

    #[test]
    fn to_length() {
        let arr = vec!["hello", "world", "this", "is", "a", "test"];
        
        fn to_len(s: &str) -> usize {
            s.len()
        }

        let result = map(arr, to_len);
        assert_eq!(result, vec![5, 5, 4, 2, 1, 4]);
    }

    #[test]
    fn to_length_with_closure() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.len());
        assert_eq!(result, vec![5, 4, 3, 8, 8]);
    }

    #[test]
    fn take_target_length_letters() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.chars().take(3).collect::<String>());
        assert_eq!(result, vec!["app", "boa", "cat", "din", "ele"]);
    }

    #[test]
    fn only_length_3_or_more_than_3() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.len() == 3);
        assert_eq!(result, vec![false, false, true, false, false]);
    }

    #[test]
    fn str_convert_to_uppercase() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.to_uppercase());
        assert_eq!(
            result, 
            vec!["APPLE", "BOAT", "CAT", "DINOSAUR", "ELEPHANT"],
        );
    }

    #[test]
    fn reverse_string() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.chars().rev().collect::<String>());
        assert_eq!(
            result, 
            vec!["elppa", "taob", "tac", "ruasonid", "tnahpele"],
        );
    }
}

#[cfg(test)]
mod fold_tests {
    use crate::functions::fold;

    #[test]
    fn basic_fold() {
        let arr = vec![1, 2, 3];
        let folded = fold(arr, 0, |acc, x| acc + x);
        assert_eq!(folded, 6);
    }

    #[test]
    fn fold_on_json_like_data_structure() {
        #[derive(Debug, PartialEq)]
        enum JsonValue {
            Boolean(bool),
            Number(f64),
            String(String),
            Array(Vec<JsonValue>),
            Object(Vec<(String, JsonValue)>),
        }

        // Number
        let input = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
        ]);

        let result = match input {
            JsonValue::Array(elm) => {
                fold(elm, 0.0, |acc, x| match x {
                    JsonValue::Number(n) => acc + n,
                    _ => acc,
                })
            },
            
            _ => 0.0,
        };

        assert_eq!(result, 6.0);

        // Boolean
        let input = JsonValue::Array(vec![
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
            JsonValue::Boolean(false),
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
            JsonValue::Boolean(true),
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
            JsonValue::Boolean(false),
            JsonValue::Boolean(true),
            JsonValue::Boolean(false),
            JsonValue::Boolean(true),
        ]);

        let result = match input {
            JsonValue::Array(elm) => {
                fold(elm, true, |acc, x| match x {
                    JsonValue::Boolean(b) => {
                        if b {
                            acc && b
                        } else {
                            !acc
                        }
                    },

                    _ => acc,
                })
            },
            
            _ => false,
        };

        assert_eq!(result, true);

        // String
        let input = JsonValue::Array(vec![
            JsonValue::String("hello".to_string()),
            JsonValue::String("world".to_string()),
            JsonValue::String("this".to_string()),
            JsonValue::String("is".to_string()),
            JsonValue::String("a".to_string()),
            JsonValue::String("test".to_string()),
        ]);

        let result = match input {
            JsonValue::Array(elm) => {
                fold(elm, "".to_string(), |acc, x| match x {
                    JsonValue::String(s) => acc + &s,
                    _ => acc,
                })
            },
            
            _ => "".to_string(),
        };

        assert_eq!(result, "helloworldthisisatest");

        // Object
        let input = JsonValue::Array(vec![
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::Number(1.0)),
                ("value".to_string(), JsonValue::Number(100.0)),
            ]),
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::Number(2.0)),
                ("value".to_string(), JsonValue::Number(200.0)),
            ]),
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::Number(3.0)),
                ("value".to_string(), JsonValue::Number(300.0)),
            ]),
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::Number(4.0)),
                ("value".to_string(), JsonValue::Number(400.0)),
            ]),
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::Number(5.0)),
                ("value".to_string(), JsonValue::Number(500.0)),
            ]),
        ]);

        let result = match input {
            JsonValue::Array(elm) => {
                fold(elm, 0.0, |acc, x| match x {
                    JsonValue::Object(obj) => {
                        let mut sum = 0.0;
                        for (_, v) in obj {
                            match v {
                                JsonValue::Number(n) => sum += n,
                                _ => (),
                            }
                        }
                        acc + sum
                    },

                    _ => acc,
                })
            },
            
            _ => 0.0,
        };

        assert_eq!(result, 1515.0);
    }
}

// #[cfg(test)]
// mod flat_tests {
//     use crate::functions::flat;

//     #[test]
//     fn basic_flat_test() {
//         let arr1: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
//         let expected1 = vec![1, 2, 3, 4];

//         assert_eq!(flat(arr1), expected1);

//         let arr2: Vec<Vec<i32>> = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
//         let expected2 = vec![1, 2, 3, 4, 5, 6];

//         assert_eq!(flat(arr2), expected2);
//     }

//     #[test]
//     fn flat_empty_vec() {
//         let arr: Vec<Vec<i32>> = vec![vec![], vec![], vec![]];
//         let expected: Vec<i32> = vec![];

//         assert_eq!(flat(arr), expected);
//     }
// }

#[cfg(test)]
mod flat_tests {
    use crate::functions::flatten;

    #[test]
    fn test_basic_flat() {
        let arr = vec![vec![1, 2, 3], vec![4, 5], vec![6]];
        let expected = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(flatten(arr), expected);
    }
}

#[cfg(test)]
mod flat_map_tests {
    use crate::functions::flat_map;

    #[test]
    fn flat_map_basic() {
        let arr = vec![1, 2, 3];
        let fm = flat_map(arr, |x| vec![x, x*2, x*3]);
        assert_eq!(fm, vec![1, 2, 3, 2, 4, 6, 3, 6, 9]);
    }

    #[test]
    fn flat_map_takes_string_vec() {
        let arr = vec!["hello", "world"];
        let result = flat_map(arr, |s| {
            s.chars().collect::<Vec<char>>()
        });
        let expected = vec!['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'];
        assert_eq!(result, expected);
    }
}