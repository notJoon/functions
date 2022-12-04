// Implementing Higher-Order Functions in rust from scratch.

///`filter` function takes in a collection of elements and returns a new collection
/// containing only the elements that satisfy a certain condition(e.g. even numbers)
/// 
/// ## Example
/// ```
/// let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let even_numbers = filter(&numbers, |&x| x % 2 == 0);
/// 
/// assert_eq!(even_numbers, vec![2, 4, 6, 8, 10]);
/// ```
pub fn filter<F, T, U>(arr: Vec<T>, callback: F) -> Vec<U> 
where 
    F: Fn(T) -> Option<U>
{
    let mut result = Vec::new();

    for e in arr {
        if let Some(mapped) = callback(e) {
            // if the callback function returns `Some`,
            // then add the unwrapped value to the result.
            result.push(mapped);
        }
    }

    result
}

/// basic `for_each` function which does not mutate or consume the values
/// captured by the closure, this function is implemented by `Fn` trait.
/// most restrictive function in `for_each` family.
/// 
/// ## Example
/// ```
/// let arr = [1, 2, 3];
/// for_each(&arr, |x| println!("{}", x));
/// ```
/// above example will print the following:
/// ```text
/// 1
/// 2
/// 3
/// ```
pub fn for_each<T, F>(arr: &Vec<T>, callback: F)
where
    F: Fn(&T),
{
    for e in arr {
        callback(e);
    }
}

/// `for_each` is implemented by `FnMut`
/// that allows the closure to mutate the values it captures.
/// 
/// If want to modify the elements of the collection within the closure.
/// 
/// ## Example
/// ```
/// let mut result = Vec::new();
/// let arr = vec![1, 2, 3, 4, 5];
///
/// for_each_mutable(&arr, |x| result.push(x * 2));
/// assert_eq!(result, vec![2, 4, 6, 8, 10]);
/// ```
pub fn for_each_mutable<T, F>(arr: &Vec<T>, mut callback: F)
where 
    F: FnMut(&T)
{
    for e in arr {
        callback(e);
    }
}

/// `map` function takes in a collection of elements and a `callback` function,
/// and returns a new collection by applying the callback function to each input element.
/// 
/// For example, a map function for a list of numbers might take in a callback function 
/// that specifies how each element should be transformed (e.g. by adding 1 to each element), 
/// and return a new list containing the transformed elements.
/// 
/// callback can be a function or closure.
/// 
/// ## Example
/// with closure:
/// ```
/// let with_closure = map(&[1, 2, 3, 4, 5], |x| x + 1);
/// ```
/// 
/// with function:
/// ```
/// fn add_one(x: i32) -> i32 { x + 1 }
/// let with_function = map(&[1, 2, 3, 4, 5], add_one);
/// ```
/// both of the above examples are same.
pub fn map<F, T, U>(arr: Vec<T>, callback: F) -> Vec<U> 
where 
    F: Fn(T) -> U
{   
    let capacity = arr.len();
    let mut result = Vec::with_capacity(capacity);

    for e in arr {
        result.push(callback(e));
    }

    result
}

/// allows to combine all the elements of a collection into a single value 
/// by applying a binary operation to each element in turn.
/// 
/// also, called `reduce` bur `fold` is meaning for the folding that the elements of the
/// collection into a single value.
/// 
/// ## Example
/// ```
/// let arr = vec![1, 2, 3, 4, 5];
/// let sum = fold(&arr, 0, |acc, x| acc + x);
/// assert_eq!(sum, 15);
/// ```
fn fold<F, T, U>(arr: Vec<T>, init: U, callback: F) -> U 
where 
    F: Fn(U, T) -> U,
{
    let mut result = init;

    for e in arr {
        result = callback(result, e);
    }

    result
}

/// create a new array with all sub-array elements concatenated
/// into it recursively up to the specified depth.
/// 
/// for example, if you have an array with elements that are also arrays,
/// `flat()` can be used to "flatten" the elements of the array into a single,
///  one-dimensional array
/// 
/// ## Example
/// ```
/// let arr = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
/// let flat = flat(&arr, 1);
/// assert_eq!(flat, vec![1, 2, 3, 4, 5, 6]);
/// ```
// TODO add depth 
pub fn flat<T>(arr: Vec<Vec<T>>, depth: usize) -> Vec<T> {
    // let mut result = Vec::new();

    // for subvec in arr {
    //     match depth {
    //         0 => {
    //             result.push(subvec);
    //         },
    //         _ => {
    //             result.push(flat(subvec, depth - 1));
    //         }
    //     }
    // }

    // result
    unimplemented!();
}

/// takes a function and an iterable as input and returns an iterator 
/// that flattens the output of the function.
/// 
/// This means that if the function returns a `list`, the `flat_map` function
/// will flatten that list into a single list of values.
/// 
/// this can be useful when you have a list of lists and you want to flatten it
/// into a single list.
/// 
/// ## Example
/// ```
/// let arr = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
/// let flat_map = flat_map(&arr, |x| x + 1);
/// 
/// assert_eq!(flat_map, vec![2, 3, 4, 5, 6, 7]);
/// ```
pub fn flat_map<F, T, U>(arr: Vec<T>, callback: F) -> Vec<U> 
where 
    F: Fn(T) -> Vec<U>,
{
    let mut result = Vec::new();
    for e in arr {
        let mapped = callback(e);
        // below code is same as 
        // `result.extend(mapped);`
        for sub in mapped {
            result.push(sub);
        }
    }
    result
}

// Tests

#[cfg(test)]
mod filter_test {
    use super::filter;

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
    use super::{for_each, for_each_mutable};

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
    use super::map;

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
    use super::fold;

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
//     use super::flat;

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
mod flat_map_tests {
    use super::flat_map;

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