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
/// callback(fn)
///  - callback function have following arguments.
///   * acc: 
///     accumulator. 
///     The value resulting from the previous iteration. On first call,
///     `init_value` if specified, otherwise `arr[0]`.
/// 
///   * current_value:
///     The current element being processed in the array.
///     On first call, the value of `arr[0]` if `init_value` was specified,
///     otherwise `arr[1]`.
/// 
///   * init_value(optional):
///     value to use as the first argument to the first call of the callback.
///     if `init_value` is supplied, that also causes `current_value` to be
///     initialized to the first value in the array.
/// 
///     Otherwise, `acc` is initialized to the first value in the array,
///     and `current_value` is initialized to the second.
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

    let mut acc = match init_value {
        Some(v) => v,
        None => arr[0],
    };

    let start = match init_value {
        Some(_) => 0,
        None => 1,
    };

    for i in start..arr.len() {
        acc = callback(acc, arr[i]);
    }
    
    Some(acc)
}

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

pub fn for_each<T>() {
    unimplemented!("for_each")
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
        struct JsonArray {
            id: i32
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
    use super::for_each;

    #[test]
    fn for_each_test() {
        unimplemented!("for_each_test")
    }
}