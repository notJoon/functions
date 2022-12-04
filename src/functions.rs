// * Implementing Some most used functions in the functional programming style
// * every functions are coded in a rust.
// *
// * All the functions are ordered in alphabetical order
// * and take description from the rust "Iterator trait docs" and "mdn web docs" website and reference the test cases.
// * function argument must be named as "callback".
// * 
// * [*] I'm not sure is it the right way to implement the functions but I think it's a good way to learn rust.
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
/// 
/// also, this method is generic. it only expects the value to have length property and integer-keyed properties.
pub fn filter_v1<T>(arr: Vec<T>, callback: fn(&T) -> bool) -> Vec<T> 
where 
    T: Copy
{
    let mut result = Vec::new();

    for k in arr.iter() {
        if callback(k) {
            result.push(*k);
        }
    }

    result
}

/// just `filter` do same works
pub fn filter_map<F, T, U>(arr: Vec<T>, callback: F) -> Vec<U> 
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

/// `For Each`
/// this method executes a provided function once for each array element.
/// It calls `callback` function once for each element in an array in ascending-index order.
/// 
/// it looks similar like `map` but `filter_map` must returns undefined result and is not chainable.
/// 
/// `for_each` does not mutate the array on which it is called, but the function
/// provided as `callback` can. the length of the array is saved before 
/// the first invocation of `callback`
/// - `callback` will not visit any elements added beyond the array's initial length
/// - changes to already-visited indexed do not cause `callback` to be invoked on them again.
/// - If an existing, yet-unvisited element of the array is changed by `callback`,
/// Its value passes to the `callback` will be the value at the time that element 
/// gets visited. `deleted` element are the visited.
/// 
/// `for_each` method is generic. it only expects the value to have a `length` property
/// and integer-keyed properties.
/// 
/// expects a synchronous function - it does not wait for promises.
/// 
/// syntax: for_each(array, callback) -> undefined
/// 
/// ref: https://docs.rs/crate/foreach/latest/source/src/lib.rs
/// 
/// ! must change this implementation 
use std::iter::Iterator;

pub enum Next {
    /// Default value, it does not change anything.
    Continue,
    /// If this, finishing iteration.
    Break,
}

impl Default for Next {
    fn default() -> Self {
        Next::Continue
    }
}

impl From<()> for Next {
    fn from(_: ()) -> Self {
        Next::Continue
    }
}

pub trait ForEach: Iterator {
    /// iterates over all items and executes given closure.
    /// 
    /// this allows to use iterator inside iteration loop,
    /// which is illegal when using `for..in` loop.
    fn for_each<N, F>(&mut self, callback: F)
    where 
        N: Into<Next>,
        F: FnMut(Self::Item, &mut Self) -> N;
}

impl<T: Iterator> ForEach for T {
    fn for_each<N, F>(&mut self, mut callback: F)
    where 
        N: Into<Next>,
        F: FnMut(Self::Item, &mut Self) -> N
    {
        while let Some(item) = self.next() {
            match callback(item, self).into() {
                Next::Continue => continue,
                Next::Break => break,
            }
        }
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
    use super::{filter_v1, filter_map};

    #[test]
    fn filter_is_big_enough() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = filter_v1(arr, |x| x > &3);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn find_all_prime_numbers() {
        let arr = vec![
            -3, -2, -1, 0, 
            1, 2, 3, 4, 5, 
            6, 7, 8, 9, 10
        ];

        let result = filter_v1(arr, |num| {
            if *num <= 1 {
                return false;
            }

            for i in 2..*num {
                if *num % i == 0 {
                    return false;
                }
            }

            true
        });

        assert_eq!(result, vec![2, 3, 5, 7]);
    }

    #[test]
    fn filter_json() {
        #[derive(Debug, PartialEq, Clone, Copy)]
        struct JsonArray<T> {
            id: T
        }

        let arr = vec![
            JsonArray {id: 15},
            JsonArray {id: -1},
            JsonArray {id: 0},
            JsonArray {id: 3},
            JsonArray {id: 12.2 as i32},
        ];

        let result = filter_v1(arr, |x| x.id > 0);
        assert_eq!(result, vec![JsonArray {id: 15}, JsonArray {id: 3}, JsonArray {id: 12}]);
    }

    #[test]
    fn test_filter_map() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = filter_map(arr, |x| {
            match x % 2 {
                0 => Some(x * 2),
                _ => None,
            }
        });

        assert_eq!(result, vec![4, 8]);
    }
}

#[cfg(test)]
mod for_each_tests {
    //! this test will be remove 
    #[test]
    fn for_each_test() {
        let iter = (0..10).into_iter();
        let mut counter = 0;
        iter.for_each(|x| {
            // println!("{}", x);
            counter += 1;
        });

        assert_eq!(counter, 10);
    }

    #[test]
    fn for_each_add() {
        let ratings = [5, 4, 5].into_iter();
        let mut total = 0;

        let sum = |a: i32, b: i32| a + b;
        ratings.for_each(|x| {
            total = sum(total, x);
        });

        assert_eq!(total, 14);
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