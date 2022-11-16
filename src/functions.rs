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
///  - should I take function parameter as a closure? 
pub fn map<T, U>(arr: Vec<T>, callback: fn(&T) -> U) -> Vec<U>{
    let mut result = Vec::new();
    for (_, v) in arr.iter().enumerate() {
        result.push(callback(v));
    }
    result
}

pub fn reduce<T>() -> T {
    unimplemented!("reduce")
}

pub fn for_each<T>() -> T {
    unimplemented!("for_each")
}

#[cfg(test)]

mod tests {

    use super::*; 

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