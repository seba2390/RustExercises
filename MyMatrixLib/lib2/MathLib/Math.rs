
/// Computes the absolute maximum value in a given slice of elements.
///
/// # Arguments
///
/// * `slice` - A slice of elements to search for the absolute maximum value.
///
/// # Returns
///
/// The absolute maximum value found in the slice. If the slice is empty, returns the default value of the element type.
///
/// # Examples
///
/// ```
/// use my_lib::abs_max;
///
/// let v = vec![0.5, -1.0, 3.0, 4.0, -2.0];
/// assert_eq!(abs_max(&v), 4.0);
///
/// let v = vec![-1, -3, -2, -4];
/// assert_eq!(abs_max(&v), 4);
/// ```
///
/// # Type parameters
///
/// * `T` - The element type of the slice. Must implement the `std::ops::Add`, `std::ops::Sub`, `std::ops::Mul`,
/// `std::ops::Div`, `std::default::Default`, `std::clone::Clone`, `std::marker::Copy`, `num_traits::Zero`, and
/// `num_traits::One` traits.
///
/// # Panics
///
/// This function will panic if the element type does not have a valid zero or one value.
pub fn abs_max<T>(slice: &[T]) -> T
    where
        T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::default::Default
        + std::clone::Clone
        + std::marker::Copy
        + num_traits::Zero
        + num_traits::One
        + num_traits::Float
{
    slice.iter().map(|x| x.abs()).fold(T::zero(), |a, b| a.max(b))
}



/// Returns the index of the absolute maximum value in the given slice.
///
/// # Arguments
///
/// * `slice`: A slice of values of any type that implements the necessary traits.
///
/// # Example
///
/// ```
/// use matrix_math::arg_abs_max;
///
/// let a = vec![-1, 2, -3, 4];
/// let idx = arg_abs_max(&a);
/// assert_eq!(idx, 2);
/// ```
///
/// # Explanation
///
/// This function iterates over each element in the input slice and creates a new iterator that maps each element
/// to a tuple containing its absolute value and its index. The `fold` function is then used to compare the
/// absolute value of each element to the previous maximum, and keep track of the index of the maximum element.
/// Finally, the function returns the index of the maximum element.
///
/// This function assumes that the input slice is not empty.
pub fn arg_abs_max<T>(slice: &[T]) -> usize
    where
        T: std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::default::Default
        + std::clone::Clone
        + std::marker::Copy
        + num_traits::Zero
        + num_traits::One
        + num_traits::Signed
        + PartialOrd,
{
    slice.iter()
        .enumerate()
        .map(|(i, x)| (x.abs(), i))
        .fold((T::zero(), 0), |(a, i), (b, j)| if a >= b {(a, i)} else {(b, j)})
        .1
}
