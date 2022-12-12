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

    for element in arr {
        if let Some(mapped) = callback(element) {
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
    for element in arr {
        callback(element);
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
    for element in arr {
        callback(element);
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

    for element in arr {
        result.push(callback(element));
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
pub fn fold<F, T, U>(arr: Vec<T>, init: U, callback: F) -> U 
where 
    F: Fn(U, T) -> U,
{
    let mut result = init;

    for element in arr {
        result = callback(result, element);
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
//TODO: support for `depth` (usize)
pub fn flatten<T>(arr: Vec<Vec<T>>) -> Vec<T> {
    let mut result = Vec::new();

    for element in arr {
        if element.is_empty() {
            continue;
        }

        for item in element {
            result.push(item);
        }
    }
    result
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
    for element in arr {
        let mapped = callback(element);
        // below code is same as 
        // `result.extend(mapped);`
        for sub in mapped {
            result.push(sub);
        }
    }
    result
}
