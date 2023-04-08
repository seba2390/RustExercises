////////////////////////////////////////////////////////////////////////////////////////////////////
///// Testing initialization methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 /////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use MyMatrixLib::Matrix;

    fn test_matrix_ones_creation_<T>()
        where
            T:
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        let ones_mat = Matrix::<T>::ones(2,2);
        assert_eq!(ones_mat[(0, 0)], T::one());
        assert_eq!(ones_mat[(0, 1)], T::one());
        assert_eq!(ones_mat[(1, 0)], T::one());
        assert_eq!(ones_mat[(1, 1)], T::one());
    }

    #[test]
    fn test_matrix_ones_creation() {
        test_matrix_ones_creation_::<i8>();
        test_matrix_ones_creation_::<i16>();
        test_matrix_ones_creation_::<i32>();
        test_matrix_ones_creation_::<i64>();
        test_matrix_ones_creation_::<u8>();
        test_matrix_ones_creation_::<u16>();
        test_matrix_ones_creation_::<u32>();
        test_matrix_ones_creation_::<u64>();
        test_matrix_ones_creation_::<f32>();
        test_matrix_ones_creation_::<f64>();
    }

    fn test_matrix_zeros_creation_<T>()
        where
            T:
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        let ones_mat = Matrix::<T>::zeros(2,2);
        assert_eq!(ones_mat[(0, 0)], T::zero());
        assert_eq!(ones_mat[(0, 1)], T::zero());
        assert_eq!(ones_mat[(1, 0)], T::zero());
        assert_eq!(ones_mat[(1, 1)], T::zero());
    }

    #[test]
    fn test_matrix_zeros_creation() {
        test_matrix_zeros_creation_::<i8>();
        test_matrix_zeros_creation_::<i16>();
        test_matrix_zeros_creation_::<i32>();
        test_matrix_zeros_creation_::<i64>();
        test_matrix_zeros_creation_::<u8>();
        test_matrix_zeros_creation_::<u16>();
        test_matrix_zeros_creation_::<u32>();
        test_matrix_zeros_creation_::<u64>();
        test_matrix_zeros_creation_::<f32>();
        test_matrix_zeros_creation_::<f64>();
    }

    fn test_matrix_identity_creation_<T>()
        where
            T:
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        let ones_mat = Matrix::<T>::identity(2);
        assert_eq!(ones_mat[(0, 0)], T::one());
        assert_eq!(ones_mat[(0, 1)], T::zero());
        assert_eq!(ones_mat[(1, 0)], T::zero());
        assert_eq!(ones_mat[(1, 1)], T::one());
    }

    #[test]
    fn test_matrix_identity_creation() {
        test_matrix_identity_creation_::<i8>();
        test_matrix_identity_creation_::<i16>();
        test_matrix_identity_creation_::<i32>();
        test_matrix_identity_creation_::<i64>();
        test_matrix_identity_creation_::<u8>();
        test_matrix_identity_creation_::<u16>();
        test_matrix_identity_creation_::<u32>();
        test_matrix_identity_creation_::<u64>();
        test_matrix_identity_creation_::<f32>();
        test_matrix_identity_creation_::<f64>();
    }
}
