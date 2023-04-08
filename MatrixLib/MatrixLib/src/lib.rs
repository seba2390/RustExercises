/// A generic Matrix type that supports basic matrix operations.
///
/// # Type Parameters
///
/// - `T`: The type of data stored in the matrix. It must implement the `Add`, `Sub`, and `Mul`
///        traits to support basic matrix operations, as well as the `Clone` and `Default` traits
///        to allow creating and copying matrices.
///
/// # Fields
///
/// - `data`: A flattened 1D array of matrix data.
/// - `rows`: The number of rows in the matrix.
/// - `cols`: The number of columns in the matrix.
///
/// #[derive(Debug, PartialEq)] is a Rust attribute that automatically generates implementations
/// of the Debug and PartialEq traits for a struct or an enum. Debug is a trait that allows you
/// to print out a struct or an enum in a formatted way using the {:?} format specifier. It is
/// useful for debugging purposes. PartialEq is a trait that defines the equality comparison
/// operation between two values. It is used to check if two instances of a struct or an enum
/// are equal or not
#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T>
    where
        T: std::ops::Add<Output=T> +
           std::ops::Sub<Output=T> +
           std::ops::Mul<Output=T> +
           std::ops::Div<Output=T> +
           std::clone::Clone +
           std::marker::Copy +
           std::default::Default +
           num_traits::Zero +
           num_traits::One
{
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// STANDARD METHODS ///////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

/// Implements the `fmt::Display` trait for the `Matrix` struct, allowing matrices to be
/// printed using the `println!` macro. The matrix is printed row by row, with each element
/// separated by a space character. A newline character is inserted after each row.
///
/// # Examples
///
/// ```
/// let mat = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
/// println!("{}", mat);
/// // Output:
/// // 1 2
/// // 3 4
/// ```
impl<T> std::fmt::Display for Matrix<T>
    where
        T:
        std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::fmt::Display +
        std::default::Default +
        std::fmt::Display +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..self.rows {
            for j in 0..self.cols {
                write!(f, "{} ", self[(i, j)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}



/// Implements the addition operation for matrices. The matrices must have the same dimensions.
///
/// # Examples
///
/// ```
/// use matrix::Matrix;
///
/// let a = Matrix::new(2, 2).fill(1.0);
/// let b = Matrix::new(2, 2).fill(2.0);
/// let c = a + b;
/// assert_eq!(c[(0, 0)], 3.0);
/// assert_eq!(c[(0, 1)], 3.0);
/// assert_eq!(c[(1, 0)], 3.0);
/// assert_eq!(c[(1, 1)], 3.0);
/// ```
///
/// # Panics
///
/// This function will panic if the dimensions of the matrices are not the same.
///
/// # Arguments
///
/// * `self` - The matrix on the left-hand side of the addition.
/// * `other` - The matrix on the right-hand side of the addition.
///
/// # Returns
///
/// A new `Matrix` object representing the result of the addition operation.
impl<T> std::ops::Add for Matrix<T>
    where
        T:
        std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::fmt::Display +
        std::default::Default +
        std::fmt::Display +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        // Check that the dimensions of both matrices match
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        // Create a new matrix with the same dimensions as self and other
        let mut result = Matrix::new(self.rows, self.cols);

        // Add the corresponding elements of self and other to get the elements of the result
        for i in 0..self.rows {
            for j in 0..self.cols {
                let val = self[(i, j)].clone() + other[(i, j)].clone();
                result[(i, j)] = val;
            }
        }
        result
    }
}

/// Implements the subtraction operator for matrices.
///
/// Given two matrices `self` and `other`, this function returns a new matrix whose elements are
/// the difference between the corresponding elements of `self` and `other`.
///
/// # Arguments
///
/// * `self` - The first matrix to subtract.
/// * `other` - The second matrix to subtract.
///
/// # Panics
///
/// This function panics if the dimensions of the two matrices do not match.
///
/// # Examples
///
/// ```
/// let a = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]);
/// let b = Matrix::from_vec(2, 2, vec![4, 3, 2, 1]);
///
/// let result = a - b;
/// assert_eq!(result, Matrix::from_vec(2, 2, vec![-3, -1, 1, 3]));
/// ```
impl<T> std::ops::Sub for Matrix<T>
    where
        T:
        std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::fmt::Display +
        std::default::Default +
        std::fmt::Display +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        // Check that the dimensions of both matrices match
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        // Create a new matrix with the same dimensions as self and other
        let mut result = Matrix::new(self.rows, self.cols);

        // Subtract the corresponding elements of other from self to get the elements of the result
        for i in 0..self.rows {
            for j in 0..self.cols {
                let val = self[(i, j)].clone() - other[(i, j)].clone();
                result[(i, j)] = val;
            }
        }
        result
    }
}


/// Multiplies two matrices together and returns the resulting matrix.
/// The number of columns in the first matrix must be equal to the number of rows in the
/// second matrix, otherwise the function will panic.
///
/// # Arguments
///
/// * `self` - The first matrix to be multiplied.
/// * `other` - The second matrix to be multiplied.
///
/// # Returns
///
/// A new matrix with dimensions (self.rows, other.cols) representing the result of the matrix
/// multiplication.
///
/// # Examples
///
/// ```
/// # use matrix_ops::Matrix;
/// let a = Matrix::from_vec(2, 3, vec![1, 2, 3, 4, 5, 6]);
/// let b = Matrix::from_vec(3, 2, vec![7, 8, 9, 10, 11, 12]);
/// let c = a * b;
/// assert_eq!(c, Matrix::from_vec(2, 2, vec![58, 64, 139, 154]));
/// ```
///
/// The above example multiplies a 2x3 matrix `a` by a 3x2 matrix `b` to get a 2x2 matrix `c`.
///
/// # Panics
///
/// This function will panic if the number of columns in the first matrix is not equal to the
/// number of rows in the second matrix.
impl<T> std::ops::Mul for Matrix<T>
    where
        T:
        std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::fmt::Display +
        std::default::Default +
        std::fmt::Display +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Check that the number of columns in the first matrix is the same as the number of rows
        // in the second matrix
        assert_eq!(self.cols, other.rows);

        // Create a new matrix with the appropriate dimensions for the result of the multiplication
        let mut result = Matrix::new(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                // Compute the dot product of the ith row of the first matrix and the jth column
                // of the second matrix
                let mut sum = T::default();
                for k in 0..self.cols {
                    sum = sum + self[(i, k)].clone() * other[(k, j)].clone();
                }
                result[(i, j)] = sum;
            }
        }

        result
    }
}


/// Provides read-only access to matrix elements using the indexing operator `[]`.
///
/// The std::ops::Index trait defines the indexing operator [] for read-only access to elements.
/// When you use matrix [(i, j)], Rust will call the std::ops::Index implementation for
/// the Matrix<T> struct, which returns a reference to the element at (i, j).
///
/// # Examples
///
/// ```
/// let matrix = Matrix::new(2, 2, vec![1, 2, 3, 4]);
/// assert_eq!(matrix[(0, 0)], 1);
/// ```
impl<T> std::ops::Index<(usize, usize)> for Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    type Output = T;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        let index = i * self.cols + j;
        &self.data[index]
    }
}


/// Provides mutable access to matrix elements using the indexing operator `[]`.
///
/// The std::ops::IndexMut trait, on the other hand, defines the indexing operator []
/// for mutable access to elements. When you use matrix[(i, j)] = value, Rust will
/// call the std::ops::IndexMut implementation for the Matrix<T> struct, which
/// returns a mutable reference to the element at (i, j).
///
///
/// # Examples
///
/// ```
/// let mut matrix = Matrix::new(2, 2, vec![1, 2, 3, 4]);
/// matrix[(0, 0)] = 5;
/// assert_eq!(matrix[(0, 0)], 5);
/// ```
impl<T> std::ops::IndexMut<(usize, usize)> for Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
        let index = i * self.cols + j;
        &mut self.data[index]
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// SPECIAL METHODS ////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////


/// Creates a new matrix of the given dimensions with default values for each element.
///
/// # Arguments
///
/// * `rows` - The number of rows in the matrix.
/// * `cols` - The number of columns in the matrix.
///
/// # Examples
///
/// ```
/// use matrix::Matrix;
///
/// let mat = Matrix::new(3, 3);
/// ```
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = vec![T::default(); rows * cols];
        Matrix { rows, cols, data }
    }
}

/// Creates a new matrix from a vector of elements with the given dimensions.
///
/// # Arguments
///
/// * `rows` - The number of rows in the matrix.
/// * `cols` - The number of columns in the matrix.
/// * `data` - A vector of elements to fill the matrix. The length of the
///            vector must be equal to `rows * cols`.
///
/// # Panics
///
/// This function will panic if the length of the `data` vector does not match the number
/// of elements in the matrix.
///
/// # Examples
///
/// ```
/// use matrix::Matrix;
///
/// let data = vec![1, 2, 3, 4, 5, 6];
/// let mat = Matrix::from_vec(2, 3, data);
/// ```
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Self {
        assert_eq!(rows * cols, data.len(), "Data length does not match matrix dimensions");

        Matrix {
            rows,
            cols,
            data,
        }
    }
}

/// Prints the contents of the matrix to the standard output stream.
///
/// The matrix is printed row by row, with each row being displayed on a separate line.
/// Elements are separated by a single space character.
/// The formatting of each element is controlled by the `std::fmt::Display` trait, which must be
/// implemented for the element type `T`.
///
/// # Examples
///
/// ```
/// # use linear_algebra::Matrix;
/// let mat = Matrix::from_vec(2, 3, vec![1, 2, 3, 4, 5, 6]);
/// mat.print();
/// // Output:
/// // 1 2 3
/// // 4 5 6
/// ```
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        std::fmt::Display +
        num_traits::Zero +
        num_traits::One
{
    /// Prints the matrix to the console.
    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self[(i, j)]);
            }
            println!("");
        }
    }
}


///////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// LINALG METHODS /////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////


/// Creates a new matrix of size `rows` by `cols` with all entries initialized to zero.
///
/// # Examples
///
/// ```
/// let m = Matrix::<f64>::zeros(3, 3);
///
/// assert_eq!(m.rows(), 3);
/// assert_eq!(m.cols(), 3);
/// assert_eq!(m[(0, 0)], 0.0);
/// assert_eq!(m[(1, 2)], 0.0);
/// assert_eq!(m[(2, 1)], 0.0);
/// ```
///
/// # Type Parameters
///
/// - `T`: The type of the matrix elements.
///
/// # Arguments
///
/// - `rows`: The number of rows in the matrix.
/// - `cols`: The number of columns in the matrix.
///
/// # Returns
///
/// A new `Matrix` of size `rows` by `cols` with all entries initialized to zero.
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let data = vec![T::zero(); rows * cols];
        Matrix { rows, cols, data }
    }
}

/// Creates a new matrix of size `rows` by `cols` with all entries initialized to one.
///
/// # Examples
///
/// ```
/// let m = Matrix::<f64>::ones(3, 3);
///
/// assert_eq!(m.rows(), 3);
/// assert_eq!(m.cols(), 3);
/// assert_eq!(m[(0, 0)], 1.0);
/// assert_eq!(m[(1, 2)], 1.0);
/// assert_eq!(m[(2, 1)], 1.0);
/// ```
///
/// # Type Parameters
///
/// - `T`: The type of the matrix elements.
///
/// # Arguments
///
/// - `rows`: The number of rows in the matrix.
/// - `cols`: The number of columns in the matrix.
///
/// # Returns
///
/// A new `Matrix` of size `rows` by `cols` with all entries initialized to one.
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    pub fn ones(rows: usize, cols: usize) -> Self {
        let data = vec![T::one(); rows * cols];
        Matrix { rows, cols, data }
    }
}


/// Constructs a new `Matrix` instance representing an identity matrix with the specified number
/// of rows and columns.
///
/// The identity matrix is a square matrix with ones on the main diagonal and zeros elsewhere.
/// It is denoted by `I` or `E`.
///
/// # Examples
///
/// ```
/// # use matrix_math::Matrix;
/// let identity = Matrix::<f64>::identity(3);
/// assert_eq!(identity, Matrix::from_vec(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]));
/// ```
///
/// # Panics
///
/// This function will panic if `rows` or `cols` is zero.
///
/// # Type Parameters
///
/// - `T`: The type of the elements in the matrix. This type must implement the `Add`, `Sub`,
/// `Mul`, `Default`, `Clone`, `num_traits::Zero`, and `num_traits::One` traits.
///
/// # Arguments
///
/// - `rows`: The number of rows in the identity matrix.
/// - `cols`: The number of columns in the identity matrix.
///
/// # Returns
///
/// A new `Matrix` instance representing an identity matrix with the specified number of
/// rows and columns.
///
/// # See Also
///
/// - `Matrix::ones`: Constructs a new `Matrix` instance with all elements set to one.
/// - `Matrix::zeros`: Constructs a new `Matrix` instance with all elements set to zero.
/// - `Matrix::from_vec`: Constructs a new `Matrix` instance from a vector
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One,
{
    pub fn identity(size: usize) -> Self {
        let mut data = vec![T::zero(); size * size];
        for i in 0..size {
            data[i * (size + 1)] = T::one();
        }
        Matrix { rows: size, cols: size, data }
    }
}



/// Computes and returns the transpose of the matrix.
///
/// Returns a new matrix with the columns and rows of the original matrix swapped.
/// The values of the original matrix are cloned into the new matrix.
///
/// # Examples
///
/// ```
///
/// let a = Matrix::from_vec(2, 3, vec![1, 2, 3, 4, 5, 6]);
/// let b = a.transpose();
/// assert_eq!(b.rows, 3);
/// assert_eq!(b.cols, 2);
/// assert_eq!(b[(0, 0)], 1);
/// assert_eq!(b[(0, 1)], 4);
/// assert_eq!(b[(1, 0)], 2);
/// assert_eq!(b[(1, 1)], 5);
/// assert_eq!(b[(2, 0)], 3);
/// assert_eq!(b[(2, 1)], 6);
/// ```
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    pub fn transpose(&self) -> Matrix<T> {
        let mut result = Matrix::new(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(j, i)] = self[(i, j)].clone();
            }
        }
        result
    }
}

/// Performs LU decomposition on the input matrix `self` and returns a tuple
/// containing the lower triangular matrix `L` and upper triangular matrix `U`.
///
/// # Panics
///
/// This method will panic if the matrix `self` is not square.
///
/// # Examples
///
/// ```
/// use matrix_ops::Matrix;
///
/// let mat = Matrix::from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 10]);
/// let (L, U) = mat.LU_decompose();
/// ```
///
/// # Notes
///
/// LU decomposition factorizes a matrix into a lower triangular matrix `L` and an
/// upper triangular matrix `U`, such that the original matrix `A` can be reconstructed
/// as `A = L * U`. The algorithm used in this implementation is the Doolittle algorithm.
///
/// The algorithm can be numerically unstable if the matrix has large condition number,
/// which can lead to large round-off errors. A permutation matrix `P` can be used to
/// mitigate this issue and yield a more stable decomposition.
///
/// This method creates a copy of the input matrix `self` to avoid mutating the original.
///
/// # References
///
/// - Doolittle algorithm: https://en.wikipedia.org/wiki/LU_decomposition#Doolittle_algorithm
/// - Numerical stability of LU decomposition: https://en.wikipedia.org/wiki/LU_decomposition#Numerical_stability
impl<T> Matrix<T>
    where
        T: std::ops::Add<Output=T> +
        std::ops::Sub<Output=T> +
        std::ops::Mul<Output=T> +
        std::ops::Div<Output=T> +
        std::default::Default +
        std::clone::Clone +
        std::marker::Copy +
        num_traits::Zero +
        num_traits::One
{
    #[allow(non_snake_case)]
    pub fn LU_decompose(&self) -> (Matrix<T>, Matrix<T>) {
        // Has to be square matrix
        assert_eq!(self.rows, self.cols);

        let mut L = Matrix::identity(self.rows);
        let mut U = (*self).clone();

        for k in 0..self.rows-1 {
            for i in k+1..self.rows {
                L[(i, k)] = U[(i, k)] / U[(k, k)];
                for j in k..self.rows {
                    U[(i, j)] = U[(i, j)] - L[(i, k)] * U[(k, j)];
                }
            }
        }
        (L, U)
    }
}

