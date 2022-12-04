// * Implementing Some most used functions in the functional programming style
// * every functions are coded in a rust.
// *
// * All the functions are ordered in alphabetical order
// * and take description from the rust "Iterator trait docs" and "mdn web docs" website and reference the test cases.
// * function argument must be named as "callback".
// * 
// * I'm not sure is it the right way to implement the functions but I think it's a good way to learn rust.
// * so, this project is only for learning purpose not for make some useful library.
// TODO: 
//   * change functions arguments to take a generic type
//   * add more functions
//   * method chaining 

/// `Filter`
/// 
/// The `filter` method is an iterative. it calls a provide callback function once for each element
/// in an array, and constructs a new array of all the values for which callback returned `true` values.
/// 
/// Array element which do not pass the callback test are not included in the new array.
pub fn filter<F, T, U>(arr: Vec<T>, callback: F) -> Vec<U> 
where 
    F: Fn(T) -> Option<U>
{
    let mut result = Vec::new();

    for e in arr {
        if let Some(mapped) = callback(e) {
            result.push(mapped);
        }
    }

    result
}

/// basic `for_each` function which does not mutate or consume the values
/// captured by the closure, this function is implemented by `Fn` trait.
/// most restrictive function in `for_each` family.
/// 
/// ```
/// let arr = [1, 2, 3, 4, 5];
/// for_each(&arr, |x| println!("{}", x));
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
/// ```
/// let arr = [1, 2, 3, 4, 5];
/// for_each_mutable(&arr, |x| *x += 1);
/// ```
pub fn for_each_mutable<T, F>(arr: &Vec<T>, mut callback: F)
where 
    F: FnMut(&T)
{
    for e in arr {
        callback(e);
    }
}

/// `Map`
/// A map represents a data structure in which collections of unique key
/// and values are stored where each key is associated with one value.
/// For example, standard array is a map where the key is the index of.
/// 
/// Usage:
///   - creates a new array from calling a function for every array element.
///   - calls a function once for each element in an array, in order.
///   - dose not execute the function for empty elements.
///   - does not change the original array.
/// 
/// TODO:
///  - should I initializing closure as a function parameter? 
///  - move test suites to other file `test.rs`
pub fn map<T, U>(arr: Vec<T>, callback: fn(&T) -> U) -> Vec<U>{
    let mut result = Vec::with_capacity(arr.len());
    
    for (_, v) in arr.iter().enumerate() {
        result.push(callback(v));
    }

    result
}

/// allows to combine all the elements of a collection into a single value 
/// by applying a binary operation to each element in turn.
/// 
/// also, called `reduce` bur `fold` is meaning for the folding that the elements of the
/// collection into a single value.
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
///  -- implementing example --
/// 
/// (1) with traits:
/// ```
/// use std::iter::{FlatMap, Map};
/// 
/// fn flat_map<F, T, U, I>(f: F, iter: T) -> FlatMap<Map<T, F>, U, I>
/// where
///     F: FnMut(U) -> I,
///     T: IntoIterator<Item = U>,
///     I: IntoIterator,
/// {
///     iter.into_iter()
///         .map(f)
///         .flat_map(|x| x)
/// }
/// 
/// let list = vec![vec![1, 2, 3], vec![4, 5, 6]];
/// let flat_list = flat_map(|x| x, list);
/// 
/// for item in flat_list {
///     println!("{}", item);
/// }
/// ```
/// 
/// (2) without trait:
/// ```
/// fn flat_map<F, T, U, I>(f: F, iter: T) -> Vec<I::Item>
/// where
///     F: FnMut(U) -> I,
///     T: IntoIterator<Item = U>,
///     I: IntoIterator,
/// {
///     iter.into_iter()
///         .map(f)
///         .flat_map(|x| x)
///         .collect::<Vec<I::Item>>()
/// }
/// ```
/// 
/// (3) without using any methods:
/// ```
/// fn flat_map<F, T, U, I>(f: F, iter: T) -> Vec<I::Item>
/// where 
///     F: FnMut(U) -> I,
///     T: IntoIterator<Item = U>,
///     I: IntoIterator,
/// {
///     let mut result = Vec::new();
///     
///     for item in iter {
///         let iter = f(item)
///         for sub in iter {
///             result.push(subitem);
///         }
///     }
///     result
/// }
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
// all tests are ordered in alphabetical order.
// also tests are grouped by function name.
// each test is named as `<function_name>_<test_name>`

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
    fn map_square() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = map(arr, |x| x * x);
        assert_eq!(result, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn map_add_one() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = map(arr, |x| x + 1);
        assert_eq!(result, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn map_take_key_value_on_struct() {
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
    fn map_str_convert_to_length() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.len());
        assert_eq!(result, vec![5, 4, 3, 8, 8]);
    }

    #[test]
    fn map_take_target_length_letters() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.chars().take(3).collect::<String>());
        assert_eq!(result, vec!["app", "boa", "cat", "din", "ele"]);
    }

    #[test]
    fn map_only_length_3_or_more_than_3() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.len() == 3);
        assert_eq!(result, vec![false, false, true, false, false]);
    }

    #[test]
    fn map_str_convert_to_uppercase() {
        let arr = vec!["apple", "boat", "cat", "dinosaur", "elephant"];
        let result = map(arr, |x| x.to_uppercase());
        assert_eq!(
            result, 
            vec!["APPLE", "BOAT", "CAT", "DINOSAUR", "ELEPHANT"],
        );
    }

    #[test]
    fn map_reverse_string() {
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
            Null, 
            Boolean(bool),
            Number(f64),
            String(String),
            Array(Vec<JsonValue>),
            Object(Vec<(String, JsonValue)>),
        }

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

            JsonValue::Null => panic!("null"),
            
            _ => 0.0,
        };

        assert_eq!(result, 6.0);
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