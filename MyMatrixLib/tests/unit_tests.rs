////////////////////////////////////////////////////////////////////////////////////////////////////
///// Testing initialization methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 /////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod initializers {
    use MyMatrixLib::Matrix;
    const MAX_SIZE: usize = 289; // Must have integer square root.
    const MIN_SIZE: usize = 1;

    fn test_matrix_new_creation_<T>()
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
        for size in MIN_SIZE..MAX_SIZE {
            let new_mat = Matrix::<T>::new(size,size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert_eq!(new_mat[(row_idx, col_idx)], T::default());
                }
            }
        }
    }

    #[test]
    fn test_matrix_new_creation() {
        test_matrix_new_creation_::<i8>();
        test_matrix_new_creation_::<i16>();
        test_matrix_new_creation_::<i32>();
        test_matrix_new_creation_::<i64>();
        test_matrix_new_creation_::<u8>();
        test_matrix_new_creation_::<u16>();
        test_matrix_new_creation_::<u32>();
        test_matrix_new_creation_::<u64>();
        test_matrix_new_creation_::<f32>();
        test_matrix_new_creation_::<f64>();
    }

    fn test_matrix_from_vec_creation_<T>()
        where
            T: std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::fmt::Display
            + num_traits::Zero
            + num_traits::One
            + num_traits::NumCast
            + std::fmt::Debug
            + PartialEq
            + Default
            + Clone
            + Copy,

    {
        'outer_loop: for root_size in MIN_SIZE..=(MAX_SIZE as f64).sqrt() as usize {
            let size = root_size * root_size;

            let mut vector: Vec<T> = vec![T::default(); size];
            for val in 0..size {
                // Doing this to stop at the point where val becomes to big to be represented by type T
                if let Some(num) = num_traits::NumCast::from(val+1).and_then(|n:T| n.into()) {
                    vector[val] = num;
                } else {
                    break 'outer_loop;
                }
            }

            let from_vec_mat = Matrix::<T>::from_vec(root_size, root_size, vector);
            let mut counter: T = T::one();
            for row_idx in 0..root_size {
                for col_idx in 0..root_size {
                    assert_eq!(from_vec_mat[(row_idx, col_idx)], counter);
                    counter = counter + T::one();
                }
            }
        }
    }

    #[test]
    fn test_matrix_from_vec_creation() {
        test_matrix_from_vec_creation_::<i8>();
        test_matrix_from_vec_creation_::<i16>();
        test_matrix_from_vec_creation_::<i32>();
        test_matrix_from_vec_creation_::<i64>();
        test_matrix_from_vec_creation_::<u8>();
        test_matrix_from_vec_creation_::<u16>();
        test_matrix_from_vec_creation_::<u32>();
        test_matrix_from_vec_creation_::<u64>();
        test_matrix_from_vec_creation_::<f32>();
        test_matrix_from_vec_creation_::<f64>();
    }


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
        for size in MIN_SIZE..MAX_SIZE {
            let ones_mat = Matrix::<T>::ones(size,size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert_eq!(ones_mat[(row_idx, col_idx)], T::one());
                }
            }
        }
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
        for size in MIN_SIZE..MAX_SIZE {
            let zeros_mat = Matrix::<T>::zeros(size,size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert_eq!(zeros_mat[(row_idx, col_idx)], T::zero());
                }
            }
        }
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
        for size in MIN_SIZE..MAX_SIZE {
            let identity_mat = Matrix::<T>::identity(size);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    if col_idx == row_idx {
                        assert_eq!(identity_mat[(row_idx, col_idx)], T::one());
                    }
                    else {
                        assert_eq!(identity_mat[(row_idx, col_idx)], T::zero());
                    }
                }
            }
        }
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


////////////////////////////////////////////////////////////////////////////////////////////////////
////////// Testing RNG methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 ///////////
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod prng_initializers {
    use MyMatrixLib::Matrix;
    const MAX_SIZE: usize = 100; // Must have integer square root.
    const MIN_SIZE: usize = 1;
    const F64_RANGE_SCALER: f64 = 0.5;
    const F32_RANGE_SCALER: f32 = 0.5;

    #[test]
    fn test_matrix_uniform_rng_creation()
    {
        for size in MIN_SIZE..MAX_SIZE {

            let rng_mat_1 =  Matrix::<i8 >::random_uniform(size, size, i8::MIN,  i8::MAX);
            let rng_mat_2 =  Matrix::<i16>::random_uniform(size, size, i16::MIN, i16::MAX);
            let rng_mat_3 =  Matrix::<i32>::random_uniform(size, size, i32::MIN, i32::MAX);
            let rng_mat_4 =  Matrix::<i64>::random_uniform(size, size, i64::MIN, i64::MAX);
            let rng_mat_5 =  Matrix::<u8 >::random_uniform(size, size, u8::MIN,  u8::MAX);
            let rng_mat_6 =  Matrix::<u16>::random_uniform(size, size, u16::MIN, u16::MAX);
            let rng_mat_7 =  Matrix::<u32>::random_uniform(size, size, u32::MIN, u32::MAX);
            let rng_mat_8 =  Matrix::<u64>::random_uniform(size, size, u64::MIN, u64::MAX);
            let rng_mat_9 =  Matrix::<f32>::random_uniform(size, size, f32::MIN*F32_RANGE_SCALER , f32::MAX*F32_RANGE_SCALER);
            let rng_mat_10 = Matrix::<f64 >::random_uniform(size, size, f64::MIN*F64_RANGE_SCALER, f64::MAX*F64_RANGE_SCALER);

            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert!(rng_mat_1[(row_idx, col_idx)]  >= i8::MIN);
                    assert!(rng_mat_1[(row_idx, col_idx)]  <  i8::MAX);

                    assert!(rng_mat_2[(row_idx, col_idx)]  >= i16::MIN);
                    assert!(rng_mat_2[(row_idx, col_idx)]  <  i16::MAX);

                    assert!(rng_mat_3[(row_idx, col_idx)]  >= i32::MIN);
                    assert!(rng_mat_3[(row_idx, col_idx)]  <  i32::MAX);

                    assert!(rng_mat_4[(row_idx, col_idx)]  >= i64::MIN);
                    assert!(rng_mat_4[(row_idx, col_idx)]  <  i64::MAX);

                    assert!(rng_mat_5[(row_idx, col_idx)]  >= u8::MIN);
                    assert!(rng_mat_5[(row_idx, col_idx)]  <  u8::MAX);

                    assert!(rng_mat_6[(row_idx, col_idx)]  >= u16::MIN);
                    assert!(rng_mat_6[(row_idx, col_idx)]  <  u16::MAX);

                    assert!(rng_mat_7[(row_idx, col_idx)]  >= u32::MIN);
                    assert!(rng_mat_7[(row_idx, col_idx)]  <  u32::MAX);

                    assert!(rng_mat_8[(row_idx, col_idx)]  >= u64::MIN);
                    assert!(rng_mat_8[(row_idx, col_idx)]  <  u64::MAX);

                    assert!(rng_mat_9[(row_idx, col_idx)]  >= f32::MIN*F32_RANGE_SCALER);
                    assert!(rng_mat_9[(row_idx, col_idx)]  <  f32::MAX*F32_RANGE_SCALER);

                    assert!(rng_mat_10[(row_idx, col_idx)]  >= f64::MIN*F64_RANGE_SCALER);
                    assert!(rng_mat_10[(row_idx, col_idx)]  <  f64::MAX*F64_RANGE_SCALER);
                }
            }
        }
    }


    fn test_matrix_bernoulli_rng_creation_<T>()
        where
            T:
            rand::distributions::uniform::SampleUniform +
            std::ops::Add<Output=T> +
            std::ops::Sub<Output=T> +
            std::ops::Mul<Output=T> +
            std::ops::Div<Output=T> +
            std::fmt::Display +
            num_traits::Zero +
            num_traits::One +
            std::fmt::Debug +
            PartialOrd +
            PartialEq +
            Default +
            Clone +
            Copy
    {
        const PROBABILITY: f64 = 0.5;
        for size in MIN_SIZE..MAX_SIZE {
            let rng_mat = Matrix::<T>::random_bernoulli(size,size,PROBABILITY);
            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert!(rng_mat[(row_idx, col_idx)] == T::zero() ||
                            rng_mat[(row_idx, col_idx)] == T::one());
                }
            }
        }
    }

    #[test]
    fn test_matrix_bernoulli_rng_creation() {
        test_matrix_bernoulli_rng_creation_::<i8>();
        test_matrix_bernoulli_rng_creation_::<i16>();
        test_matrix_bernoulli_rng_creation_::<i32>();
        test_matrix_bernoulli_rng_creation_::<i64>();
        test_matrix_bernoulli_rng_creation_::<u8>();
        test_matrix_bernoulli_rng_creation_::<u16>();
        test_matrix_bernoulli_rng_creation_::<u32>();
        test_matrix_bernoulli_rng_creation_::<u64>();
        test_matrix_bernoulli_rng_creation_::<f32>();
        test_matrix_bernoulli_rng_creation_::<f64>();
    }
}


////////////////////////////////////////////////////////////////////////////////////////////////////
/////// Testing operation methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 ////////
////////////////////////////////////////////////////////////////////////////////////////////////////


#[cfg(test)]
mod operations {
    use MyMatrixLib::Matrix;
    const MAX_SIZE: usize = 100; // Must have integer square root.
    const MIN_SIZE: usize = 1;
    const F64_RANGE_SCALER: f64 = 0.5;
    const F32_RANGE_SCALER: f32 = 0.5;

    #[test]
    fn test_matrix_addition()
    {

        // All the random upper and lower bounds as set in ways to prevent overflow.
        for mat_size in MIN_SIZE..MAX_SIZE {
            let mat_a_1 = Matrix::<i8>::random_uniform(mat_size, mat_size, -(2_i8.pow(8-2)-1) as i8, 2_i8.pow(8-2)-1 as i8);
            let mat_b_1 = Matrix::<i8>::random_uniform(mat_size, mat_size, -(2_i8.pow(8-2)-1) as i8, 2_i8.pow(8-2)-1 as i8);
            let mat_c_1 = mat_a_1.clone() + mat_b_1.clone(); // Add clone to matrices instead of consuming them

            let mat_a_2 = Matrix::<i16>::random_uniform(mat_size, mat_size, -(2_i16.pow(16-2)-1), 2_i16.pow(16-2)-1 as i16);
            let mat_b_2 = Matrix::<i16>::random_uniform(mat_size, mat_size, -(2_i16.pow(16-2)-1), 2_i16.pow(16-2)-1 as i16);
            let mat_c_2 = mat_a_2.clone() + mat_b_2.clone(); // Add clone to matrices instead of consuming them

            let mat_a_3 = Matrix::<i32>::random_uniform(mat_size, mat_size, -(2_i32.pow(32-2)-1), 2_i32.pow(32-2)-1 as i32);
            let mat_b_3 = Matrix::<i32>::random_uniform(mat_size, mat_size, -(2_i32.pow(32-2)-1), 2_i32.pow(32-2)-1 as i32);
            let mat_c_3 = mat_a_3.clone() + mat_b_3.clone(); // Add clone to matrices instead of consuming them

            let mat_a_4 = Matrix::<i64>::random_uniform(mat_size, mat_size, -(2_i64.pow(64-2)-1), 2_i64.pow(64-2)-1 as i64);
            let mat_b_4 = Matrix::<i64>::random_uniform(mat_size, mat_size, -(2_i64.pow(64-2)-1), 2_i64.pow(64-2)-1 as i64);
            let mat_c_4 = mat_a_4.clone() + mat_b_4.clone(); // Add clone to matrices instead of consuming them

            let mat_a_5 = Matrix::<u8>::random_uniform(mat_size, mat_size, u8::MIN, 2_u8.pow(8-2)-1 as u8);
            let mat_b_5 = Matrix::<u8>::random_uniform(mat_size, mat_size, u8::MIN, 2_u8.pow(8-2)-1 as u8);
            let mat_c_5 = mat_a_5.clone() + mat_b_5.clone(); // Add clone to matrices instead of consuming them

            let mat_a_6 = Matrix::<u16>::random_uniform(mat_size, mat_size, u16::MIN, 2_u16.pow(16-2)-1 as u16);
            let mat_b_6 = Matrix::<u16>::random_uniform(mat_size, mat_size, u16::MIN, 2_u16.pow(16-2)-1 as u16);
            let mat_c_6 = mat_a_6.clone() + mat_b_6.clone(); // Add clone to matrices instead of consuming them

            let mat_a_7 = Matrix::<u32>::random_uniform(mat_size, mat_size, u32::MIN, 2_u32.pow(32-2)-1 as u32);
            let mat_b_7 = Matrix::<u32>::random_uniform(mat_size, mat_size, u32::MIN, 2_u32.pow(32-2)-1 as u32);
            let mat_c_7 = mat_a_7.clone() + mat_b_7.clone(); // Add clone to matrices instead of consuming them

            let mat_a_8 = Matrix::<u64>::random_uniform(mat_size, mat_size, u64::MIN, 2_u64.pow(64-2)-1 as u64);
            let mat_b_8 = Matrix::<u64>::random_uniform(mat_size, mat_size, u64::MIN, 2_u64.pow(64-2)-1 as u64);
            let mat_c_8 = mat_a_8.clone() + mat_b_8.clone(); // Add clone to matrices instead of consuming them

            let mat_a_9 = Matrix::<f32>::random_uniform(mat_size, mat_size, 0.0, f32::MAX*0.5-1.0);
            let mat_b_9 = Matrix::<f32>::random_uniform(mat_size, mat_size, 0.0, f32::MAX*0.5-1.0);
            let mat_c_9 = mat_a_9.clone() + mat_b_9.clone(); // Add clone to matrices instead of consuming them

            let mat_a_10 = Matrix::<f64>::random_uniform(mat_size, mat_size, 0.0, f64::MAX*0.5-1.0);
            let mat_b_10 = Matrix::<f64>::random_uniform(mat_size, mat_size, 0.0, f64::MAX*0.5-1.0);
            let mat_c_10 = mat_a_10.clone() + mat_b_10.clone(); // Add clone to matrices instead of consuming them

            for row_idx in 0..mat_size {
                for col_idx in 0..mat_size {
                    assert_eq!(mat_c_1[(row_idx, col_idx)], mat_a_1[(row_idx,col_idx)] + mat_b_1[(row_idx,col_idx)]);
                    assert_eq!(mat_c_2[(row_idx, col_idx)], mat_a_2[(row_idx,col_idx)] + mat_b_2[(row_idx,col_idx)]);
                    assert_eq!(mat_c_3[(row_idx, col_idx)], mat_a_3[(row_idx,col_idx)] + mat_b_3[(row_idx,col_idx)]);
                    assert_eq!(mat_c_4[(row_idx, col_idx)], mat_a_4[(row_idx,col_idx)] + mat_b_4[(row_idx,col_idx)]);
                    assert_eq!(mat_c_5[(row_idx, col_idx)], mat_a_5[(row_idx,col_idx)] + mat_b_5[(row_idx,col_idx)]);
                    assert_eq!(mat_c_6[(row_idx, col_idx)], mat_a_6[(row_idx,col_idx)] + mat_b_6[(row_idx,col_idx)]);
                    assert_eq!(mat_c_7[(row_idx, col_idx)], mat_a_7[(row_idx,col_idx)] + mat_b_7[(row_idx,col_idx)]);
                    assert_eq!(mat_c_8[(row_idx, col_idx)], mat_a_8[(row_idx,col_idx)] + mat_b_8[(row_idx,col_idx)]);
                    assert_eq!(mat_c_9[(row_idx, col_idx)], mat_a_9[(row_idx,col_idx)] + mat_b_9[(row_idx,col_idx)]);
                    assert_eq!(mat_c_10[(row_idx, col_idx)], mat_a_10[(row_idx,col_idx)] + mat_b_10[(row_idx,col_idx)]);
                }
            }
        }
    }

    #[test]
    fn test_matrix_subtraction()
    {

        // All the random upper and lower bounds as set in ways to prevent overflow.
        for mat_size in MIN_SIZE..MAX_SIZE {
            let mat_a_1 = Matrix::<i8>::random_uniform(mat_size, mat_size, -(2_i8.pow(8-2)-1) as i8, 2_i8.pow(8-2)-1 as i8);
            let mat_b_1 = Matrix::<i8>::random_uniform(mat_size, mat_size, -(2_i8.pow(8-2)-1) as i8, 2_i8.pow(8-2)-1 as i8);
            let mat_c_1 = mat_a_1.clone() - mat_b_1.clone(); // Add clone to matrices instead of consuming them

            let mat_a_2 = Matrix::<i16>::random_uniform(mat_size, mat_size, -(2_i16.pow(16-2)-1), 2_i16.pow(16-2)-1 as i16);
            let mat_b_2 = Matrix::<i16>::random_uniform(mat_size, mat_size, -(2_i16.pow(16-2)-1), 2_i16.pow(16-2)-1 as i16);
            let mat_c_2 = mat_a_2.clone() - mat_b_2.clone(); // Add clone to matrices instead of consuming them

            let mat_a_3 = Matrix::<i32>::random_uniform(mat_size, mat_size, -(2_i32.pow(32-2)-1), 2_i32.pow(32-2)-1 as i32);
            let mat_b_3 = Matrix::<i32>::random_uniform(mat_size, mat_size, -(2_i32.pow(32-2)-1), 2_i32.pow(32-2)-1 as i32);
            let mat_c_3 = mat_a_3.clone() - mat_b_3.clone(); // Add clone to matrices instead of consuming them

            let mat_a_4 = Matrix::<i64>::random_uniform(mat_size, mat_size, -(2_i64.pow(64-2)-1), 2_i64.pow(64-2)-1 as i64);
            let mat_b_4 = Matrix::<i64>::random_uniform(mat_size, mat_size, -(2_i64.pow(64-2)-1), 2_i64.pow(64-2)-1 as i64);
            let mat_c_4 = mat_a_4.clone() - mat_b_4.clone(); // Add clone to matrices instead of consuming them

            let mat_a_5 = Matrix::<u8>::random_uniform(mat_size, mat_size, 2_u8.pow(8-2), u8::MAX);
            let mat_b_5 = Matrix::<u8>::random_uniform(mat_size, mat_size, u8::MIN, 2_u8.pow(8-2)-1 as u8);
            let mat_c_5 = mat_a_5.clone() - mat_b_5.clone(); // Add clone to matrices instead of consuming them

            let mat_a_6 = Matrix::<u16>::random_uniform(mat_size, mat_size, 2_u16.pow(16-2), u16::MAX);
            let mat_b_6 = Matrix::<u16>::random_uniform(mat_size, mat_size, u16::MIN, 2_u16.pow(16-2)-1 as u16);
            let mat_c_6 = mat_a_6.clone() - mat_b_6.clone(); // Add clone to matrices instead of consuming them

            let mat_a_7 = Matrix::<u32>::random_uniform(mat_size, mat_size, 2_u32.pow(32-2), u32::MAX);
            let mat_b_7 = Matrix::<u32>::random_uniform(mat_size, mat_size, u32::MIN, 2_u32.pow(32-2)-1 as u32);
            let mat_c_7 = mat_a_7.clone() - mat_b_7.clone(); // Add clone to matrices instead of consuming them

            let mat_a_8 = Matrix::<u64>::random_uniform(mat_size, mat_size, 2_u64.pow(64-2), u64::MAX);
            let mat_b_8 = Matrix::<u64>::random_uniform(mat_size, mat_size, u64::MIN, 2_u64.pow(64-2)-1 as u64);
            let mat_c_8 = mat_a_8.clone() - mat_b_8.clone(); // Add clone to matrices instead of consuming them

            let mat_a_9 = Matrix::<f32>::random_uniform(mat_size, mat_size, 0.0, f32::MAX*0.5-1.0);
            let mat_b_9 = Matrix::<f32>::random_uniform(mat_size, mat_size, 0.0, f32::MAX*0.5-1.0);
            let mat_c_9 = mat_a_9.clone() - mat_b_9.clone(); // Add clone to matrices instead of consuming them

            let mat_a_10 = Matrix::<f64>::random_uniform(mat_size, mat_size, 0.0, f64::MAX*0.5-1.0);
            let mat_b_10 = Matrix::<f64>::random_uniform(mat_size, mat_size, 0.0, f64::MAX*0.5-1.0);
            let mat_c_10 = mat_a_10.clone() - mat_b_10.clone(); // Add clone to matrices instead of consuming them

            for row_idx in 0..mat_size {
                for col_idx in 0..mat_size {
                    assert_eq!(mat_c_1[(row_idx, col_idx)], mat_a_1[(row_idx,col_idx)] - mat_b_1[(row_idx,col_idx)]);
                    assert_eq!(mat_c_2[(row_idx, col_idx)], mat_a_2[(row_idx,col_idx)] - mat_b_2[(row_idx,col_idx)]);
                    assert_eq!(mat_c_3[(row_idx, col_idx)], mat_a_3[(row_idx,col_idx)] - mat_b_3[(row_idx,col_idx)]);
                    assert_eq!(mat_c_4[(row_idx, col_idx)], mat_a_4[(row_idx,col_idx)] - mat_b_4[(row_idx,col_idx)]);
                    assert_eq!(mat_c_5[(row_idx, col_idx)], mat_a_5[(row_idx,col_idx)] - mat_b_5[(row_idx,col_idx)]);
                    assert_eq!(mat_c_6[(row_idx, col_idx)], mat_a_6[(row_idx,col_idx)] - mat_b_6[(row_idx,col_idx)]);
                    assert_eq!(mat_c_7[(row_idx, col_idx)], mat_a_7[(row_idx,col_idx)] - mat_b_7[(row_idx,col_idx)]);
                    assert_eq!(mat_c_8[(row_idx, col_idx)], mat_a_8[(row_idx,col_idx)] - mat_b_8[(row_idx,col_idx)]);
                    assert_eq!(mat_c_9[(row_idx, col_idx)], mat_a_9[(row_idx,col_idx)] - mat_b_9[(row_idx,col_idx)]);
                    assert_eq!(mat_c_10[(row_idx, col_idx)], mat_a_10[(row_idx,col_idx)] - mat_b_10[(row_idx,col_idx)]);
                }
            }
        }
    }


    #[test]
    fn test_matrix_transposing()
    {

        // All the random upper and lower bounds as set in ways to prevent overflow.
        for mat_size in MIN_SIZE..MAX_SIZE {
            let rng_mat_1   =  Matrix::<i8 >::random_uniform(mat_size, mat_size, i8::MIN,  i8::MAX);
            let rng_mat_1_t =  rng_mat_1.transpose();

            let rng_mat_2 =  Matrix::<i16>::random_uniform(mat_size, mat_size, i16::MIN, i16::MAX);
            let rng_mat_2_t =  rng_mat_2.transpose();

            let rng_mat_3 =  Matrix::<i32>::random_uniform(mat_size, mat_size, i32::MIN, i32::MAX);
            let rng_mat_3_t =  rng_mat_3.transpose();

            let rng_mat_4 =  Matrix::<i64>::random_uniform(mat_size, mat_size, i64::MIN, i64::MAX);
            let rng_mat_4_t =  rng_mat_4.transpose();

            let rng_mat_5 =  Matrix::<u8 >::random_uniform(mat_size, mat_size, u8::MIN,  u8::MAX);
            let rng_mat_5_t =  rng_mat_5.transpose();

            let rng_mat_6 =  Matrix::<u16>::random_uniform(mat_size, mat_size, u16::MIN, u16::MAX);
            let rng_mat_6_t =  rng_mat_6.transpose();

            let rng_mat_7 =  Matrix::<u32>::random_uniform(mat_size, mat_size, u32::MIN, u32::MAX);
            let rng_mat_7_t =  rng_mat_7.transpose();

            let rng_mat_8 =  Matrix::<u64>::random_uniform(mat_size, mat_size, u64::MIN, u64::MAX);
            let rng_mat_8_t =  rng_mat_8.transpose();

            let rng_mat_9 =  Matrix::<f32>::random_uniform(mat_size, mat_size, f32::MIN*F32_RANGE_SCALER , f32::MAX*F32_RANGE_SCALER);
            let rng_mat_9_t =  rng_mat_9.transpose();

            let rng_mat_10 = Matrix::<f64 >::random_uniform(mat_size, mat_size, f64::MIN*F64_RANGE_SCALER, f64::MAX*F64_RANGE_SCALER);
            let rng_mat_10_t =  rng_mat_10.transpose();

            for row_idx in 0..mat_size {
                for col_idx in 0..mat_size {
                    assert_eq!(rng_mat_1[(row_idx, col_idx)], rng_mat_1_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_2[(row_idx, col_idx)], rng_mat_2_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_3[(row_idx, col_idx)], rng_mat_3_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_4[(row_idx, col_idx)], rng_mat_4_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_5[(row_idx, col_idx)], rng_mat_5_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_6[(row_idx, col_idx)], rng_mat_6_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_7[(row_idx, col_idx)], rng_mat_7_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_8[(row_idx, col_idx)], rng_mat_8_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_9[(row_idx, col_idx)], rng_mat_9_t[(col_idx, row_idx)]);
                    assert_eq!(rng_mat_10[(row_idx, col_idx)], rng_mat_10_t[(col_idx, row_idx)]);


                }
            }
        }
    }

}


////////////////////////////////////////////////////////////////////////////////////////////////////
//////// Testing linalg methods for types: i8, i16, i32, i64, u8, u16, u32, u64, f32, f64 //////////
////////////////////////////////////////////////////////////////////////////////////////////////////



#[cfg(test)]
mod linalg_operations {
    use MyMatrixLib::Matrix;
    const MAX_SIZE: usize = 50; // Must have integer square root.
    const MIN_SIZE: usize = 1;
    const F64_RANGE_SCALER: f64 = 0.5;
    const F32_RANGE_SCALER: f32 = 0.5;

    #[allow(non_snake_case)]
    //#[test] TODO: Fix bounds to avoid overflow
    fn test_matrix_LU_decomposition()
    {
        for size in MIN_SIZE..MAX_SIZE {

            let rng_mat_1 =  Matrix::<i8 >::random_uniform(size, size, -11_i8,  11_i8);
            let (l1, u1) = rng_mat_1.LU_decompose();
            let prod1 = l1 * u1;

            let rng_mat_2 =  Matrix::<i16>::random_uniform(size, size, -181_i16, 181_i16);
            let (l2, u2) = rng_mat_2.LU_decompose();
            let prod2 = l2 * u2;

            let rng_mat_3 =  Matrix::<i32>::random_uniform(size, size, -46340_i32, 46340_i32);
            let (l3, u3) = rng_mat_3.LU_decompose();
            let prod3 = l3 * u3;

            let rng_mat_4 =  Matrix::<i64>::random_uniform(size, size, -3037000499_i64, 3037000499_i64);
            let (l4, u4) = rng_mat_4.LU_decompose();
            let prod4 = l4 * u4;

            let rng_mat_5 =  Matrix::<u8 >::random_uniform(size, size, 0_u8,  11_u8);
            let (l5, u5) = rng_mat_5.LU_decompose();
            let prod5 = l5 * u5;

            let rng_mat_6 =  Matrix::<u16>::random_uniform(size, size, 0_u16, 181_u16);
            let (l6, u6) = rng_mat_6.LU_decompose();
            let prod6 = l6 * u6;

            let rng_mat_7 =  Matrix::<u32>::random_uniform(size, size, 0_u32, 46340_u32);
            let (l7, u7) = rng_mat_7.LU_decompose();
            let prod7 = l7 * u7;

            let rng_mat_8 =  Matrix::<u64>::random_uniform(size, size, 0_u64, 3037000499_u64);
            let (l8, u8) = rng_mat_8.LU_decompose();
            let prod8 = l8 * u8;

            let rng_mat_9 =  Matrix::<f32>::random_uniform(size, size, -46340.0*0.5, 46340.0*0.5);
            let (l9, u9) = rng_mat_9.LU_decompose();
            let prod9 = l9 * u9;

            let rng_mat_10 = Matrix::<f64 >::random_uniform(size, size, -3037000499.0*0.5, 3037000499.0*0.5);
            let (l10, u10) = rng_mat_10.LU_decompose();
            let prod10 = l10 * u10;

            for row_idx in 0..size {
                for col_idx in 0..size {
                    assert_eq!(rng_mat_1[(row_idx, col_idx)],prod1[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_2[(row_idx, col_idx)],prod2[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_3[(row_idx, col_idx)],prod3[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_4[(row_idx, col_idx)],prod4[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_5[(row_idx, col_idx)],prod5[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_6[(row_idx, col_idx)],prod6[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_7[(row_idx, col_idx)],prod7[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_8[(row_idx, col_idx)],prod8[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_9[(row_idx, col_idx)],prod9[(row_idx, col_idx)]);
                    assert_eq!(rng_mat_10[(row_idx, col_idx)],prod10[(row_idx, col_idx)]);
                }
            }
        }
    }


}
