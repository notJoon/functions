//* Implementing Some most used functions in the functional programming style
//* every functions are coded in a rust.
//*
//* All the functions are ordered in alphabetical order
//* and take description from the rust "Iterator trait docs" and "mdn web docs" website and reference the test cases.
//* function argument must be named as "callback".
//* 
//* [*] I'm not sure is it the right way to implement the functions but I think it's a good way to learn rust.
//* so, this project is only for learning purpose not for make some useful library.
//TODO: 
//   * change functions arguments to take a generic type
//   * add more functions
//   * change functions not take vectors as arguments, just be as an iterator-like to able method chaining.


/// `Filter`
/// 
/// The `filter` method is an iterative. it calls a provide callback function once for each element
/// in an array, and constructs a new array of all the values for which callback returned `true` values.
/// 
/// Array element which do not pass the callback test are not included in the new array.
/// 
/// also, this method is generic. it only expects the value to have length property and integer-keyed properties.
pub fn filter<T>(arr: Vec<T>, callback: fn(&T) -> bool) -> Vec<T> 
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

/// `Reduce`
/// executes a user-supplied reducer callback function on each element of the array.
/// In order, passing in the return value from the calculation on the preceding element.
/// The final result of running the reducer across all elements of the array is a single value.
/// 
/// Return: 
/// The value that results from running the reducer callback function
/// to completion over the array.
/// 
/// Error:
/// TypeError - if the given array contains no elements and no initialValue is provided.
pub fn reduce(
    arr: Vec<i32>, 
    init_value: Option<i32>, 
    callback: fn(acc: i32, cur_val: i32) -> i32
) -> Option<i32> 
{
    if arr.is_empty() {
        panic!("TypeError: empty array");
    }

    let (mut acc, start) = match init_value {
        Some(v) => (v, 0),
        None => (arr[0], 1),
    };

    for i in start..arr.len() {
        acc = callback(acc, arr[i]);
    }
    
    Some(acc)
}

// Tests
// all tests are ordered in alphabetical order.
// also tests are grouped by function name.
// each test is named as `<function_name>_<test_name>`
#[cfg(test)]
mod filter_test {
    use super::filter;

    #[test]
    fn filter_is_big_enough() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = filter(arr, |x| x > &3);
        assert_eq!(result, vec![4, 5]);
    }

    #[test]
    fn find_all_prime_numbers() {
        let arr = vec![
            -3, -2, -1, 0, 
            1, 2, 3, 4, 5, 
            6, 7, 8, 9, 10
        ];

        let result = filter(arr, |num| {
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

        let result = filter(arr, |x| x.id > 0);
        assert_eq!(result, vec![JsonArray {id: 15}, JsonArray {id: 3}, JsonArray {id: 12}]);
    }
}

#[cfg(test)]
mod for_each_tests {
    use super::ForEach;

    #[test]
    fn for_each_test() {
        let iter = (0..10).into_iter();
        let mut counter = 0;
        iter.for_each(|x| {
            println!("{}", x);
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
mod reduce_tests {
    use super::reduce;

    #[test]
    #[should_panic(expected = "TypeError: empty array")]
    fn reduce_must_panic() {
        let arr = vec![];
        let result = reduce(arr, None, |acc, x| acc + x);
        assert_eq!(result, None);
    }

    #[test]
    fn reduce_sum_without_init_value() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = reduce(arr, None, |acc, x| acc + x);
        assert_eq!(result, Some(15));
    }

    #[test]
    fn reduce_sum_with_init_value() {
        let arr = vec![1, 2, 3, 4, 5];
        let result = reduce(arr, Some(10), |acc, x| acc + x);
        assert_eq!(result, Some(25));
    }

    // #[test]
    // fn reduce_sum_with_object_array() {
    //     #[derive(Debug, PartialEq, Clone)]
    //     struct TestArray {
    //         key: i32,
    //         value: i32,
    //     }

    //     let arr = vec![
    //         TestArray { key: 1, value: 100 },
    //         TestArray { key: 2, value: 200 },
    //         TestArray { key: 3, value: 300 },
    //         TestArray { key: 4, value: 450 },
    //         TestArray { key: 5, value: 50000 },
    //     ];

    //     let result = reduce(arr, None, |acc, x| acc + x.value);
    //     assert_eq!(result, Some(50850));
    // }
}