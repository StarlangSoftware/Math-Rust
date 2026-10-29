use crate::vector::Vector;
use crate::eigenvector::Eigenvector;

pub struct Matrix{
    row: usize,
    col: usize,
    values: Vec<f64>,
}

impl Matrix {
    pub fn new(row: usize, col: usize) -> Matrix {
        Matrix{row, col, values:vec![0.0; row*col]}
    }

    pub fn clone(&self) -> Matrix {
        Matrix{row: self.row, col: self.col, values:self.values.clone()}
    }

    /// Another constructor of Matrix class which takes row, column, minimum and maximum values as inputs.
    /// First it creates new values with given row and column numbers. Then fills in the
    /// positions with random numbers using minimum and maximum inputs.
    ///
    /// # Arguments
    ///
    /// * `row`: number of rows
    /// * `col`: number of columns
    /// * `min`: minimum value.
    /// * `max`: maximum value.
    ///
    /// returns: Matrix
    ///
    pub fn random(row: usize, col: usize, min: f64, max: f64) -> Matrix {
        let mut values = vec![0.0; row * col];
        for i in 0..row*col {
            values[i]= rand::random_range(min..max);
        }
        Matrix{row, col, values }
    }

    /// Another constructor of Matrix class which takes size as input and creates new values
    /// with using size input and assigns 1 to each element at the diagonal.
    ///
    /// # Arguments
    ///
    /// * `row`: Number of rows and columns of the matrix
    ///
    /// returns: Matrix
    ///
    pub fn identity(row: usize) -> Matrix {
        let mut values = vec![0.0; row * row];
        for i in 0..row {
            values[i * row + i]= 1.0;
        }
        Matrix{row, col:row, values}
    }

    /// The getter for the index at given rowNo and colNo of values.
    ///
    /// # Arguments
    ///
    /// * `row`: integer input for row number.
    /// * `col`: integer input for column number.
    ///
    /// returns: f64 item at given index of values
    ///
    pub fn get_value(&self, row: usize, col: usize) -> f64 {
        self.values[row * self.col + col]
    }

    /// The setter for the value at given index of values
    ///
    /// # Arguments
    ///
    /// * `row`: integer input for row number.
    /// * `col`: integer input for column number.
    /// * `value`: is used to set at given index.
    ///
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        self.values[row * self.col + col] = value;
    }

    /// The addValue method adds the given value to the item at given index of values
    ///
    /// # Arguments
    ///
    /// * `row`: integer input for row number.
    /// * `col`: integer input for column number.
    /// * `value`: is used to add to given item at given index.
    ///
    pub fn add_value(&mut self, row: usize, col: usize, value: f64) {
        self.values[row * self.col + col] += value;
    }

    /// The increment method adds 1 to the item at given index of values
    ///
    /// # Arguments
    ///
    /// * `row`: integer input for row number.
    /// * `col`: integer input for column number.
    ///
    pub fn increment(&mut self, row: usize, col: usize) {
        self.values[row * self.col + col] += 1.0;
    }

    /// The getter for the row variable.
    ///
    /// returns: usize row number.
    ///
    pub fn get_row(&self) -> usize {
        self.row
    }

    /// The getRow method returns the vector of values at given row input.
    ///
    /// # Arguments
    ///
    /// * `row`: integer input for row number.
    ///
    /// returns: Vec<f64, Global> Vector of values at given row input.
    ///
    pub fn get_row_vector(&self, row: usize) -> Vec<f64> {
        self.values[row * self.col..(row+1) * self.col].to_vec()
    }

    /// The getColumn method creates a vector and adds items at given column number of values
    /// to the vector.
    ///
    /// # Arguments
    ///
    /// * `col`: integer input for column number.
    ///
    /// returns: Vec<f64, Global> vector of given column number.
    ///
    pub fn get_column_vector(&self, col: usize) -> Vec<f64> {
        let mut result: Vec<f64> = Vec::new();
        for i in 0..self.row {
            result.push(self.get_value(i, col));
        }
        result
    }

    /// The getter for the col variable.
    ///
    /// returns: usize col number.
    ///
    pub fn get_col(&self) -> usize {
        self.col
    }

    /// The columnWiseNormalize method, first accumulates items column by column then divides items by the summation.
    ///
    pub fn columnwise_normalize(&mut self) {
        for i in 0..self.row {
            let mut sum = 0.0;
            for j in 0..self.col {
                sum += self.get_value(i, j);
            }
            for j in 0..self.col {
                self.set_value(i, j, self.get_value(i, j) / sum);
            }
        }
    }

    /// The multiplyWithConstant method takes a constant as an input and multiplies each item of values {@link java.lang.reflect.Array}
    /// with given constant.
    ///
    /// # Arguments
    ///
    /// * `constant`: value to multiply items of values
    ///
    pub fn multiply_with_constant(&mut self, constant: f64) {
        for i in 0..self.row {
            for j in 0..self.col {
                self.set_value(i, j, constant * self.get_value(i, j));
            }
        }
    }

    /// The divideByConstant method takes a constant as an input and divides each item of values {@link java.lang.reflect.Array}
    /// with given constant.
    ///
    /// # Arguments
    ///
    /// * `constant`: value to divide items of values
    ///
    pub fn divide_by_constant(&mut self, constant: f64) {
        for i in 0..self.row {
            for j in 0..self.col {
                self.set_value(i, j, self.get_value(i, j) / constant);
            }
        }
    }

    /// The add method takes a Matrix as an input and accumulates values with the
    /// corresponding items of given Matrix. If the sizes of both Matrix and values do not match,
    /// it panics.
    ///
    /// # Arguments
    ///
    /// * `m`: Matrix to be added.
    ///
    pub fn add(&mut self, m: &Matrix) {
        if (self.row != m.row) || (self.col != m.col) {
            panic!("Matrix does not support adding at same row and/or column");
        }
        for i in 0..self.row {
            for j in 0..self.col {
                self.add_value(i, j, m.get_value(i, j));
            }
        }
    }

    /// The add method takes a Matrix as an input and sums values with the
    /// corresponding items of given Matrix and returns as a new Matrix. If the sizes of both Matrix and values
    ///  do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `m`: Matrix to be added.
    ///
    /// returns: Matrix Sum of current matrix and m.
    ///
    pub fn sum(&self, m: &Matrix) -> Matrix {
        if (self.row != m.row) || (self.col != m.col) {
            panic!("Matrix does not support adding at same row and column");
        }
        let mut result = Matrix::new(self.row, self.col);
        for i in 0..self.row {
            for j in 0..self.col {
                result.set_value(i, j, self.get_value(i, j) + m.get_value(i, j));
            }
        }
        result
    }

    /// The add method which takes a row number and a Vector as inputs. It sums up the corresponding values at the given row of
    /// values and given Vector. If the sizes of both Matrix and values
    //  do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `row`: integer input for row number.
    /// * `vector`: Vector type input.
    ///
    pub fn add_vector(&mut self, row: usize, vector: &Vector) {
        if self.col != vector.size(){
            panic!("Matrix number of columns should match the size of the vector");
        }
        for i in 0..self.col {
            self.add_value(row, i, vector.get_value(i));
        }
    }

    /// The subtract method takes a {@link Matrix} as an input and subtracts from values the
    /// corresponding items of given Matrix. If the sizes of both Matrix and values do not match,
    /// it panics.
    ///
    /// # Arguments
    ///
    /// * `m`: Matrix type input.
    ///
    pub fn subtract(&mut self, m: &Matrix) {
        if (self.row != m.row) || (self.col != m.col) {
            panic!("Matrix does not support subtracting at same row and column");
        }
        for i in 0..self.row {
            for j in 0..self.col {
                self.add_value(i, j, - m.get_value(i, j));
            }
        }
    }

    /// The difference method takes a Matrix as an input and subtracts from values the
    /// corresponding items of given Matrix. If the sizes of both Matrix and values do not match,
    /// it panics.
    ///
    /// # Arguments
    ///
    /// * `m`: Matrix type input.
    ///
    /// returns: Matrix Difference between current matrix and m.
    ///
    pub fn difference(&mut self, m: &Matrix) -> Matrix {
        if (self.row != m.row) || (self.col != m.col) {
            panic!("Matrix does not support subtracting at same row and column");
        }
        let mut result = Matrix::new(self.row, self.col);
        for i in 0..self.row {
            for j in 0..self.col {
                result.set_value(i, j, self.get_value(i, j) - m.get_value(i, j));
            }
        }
        result
    }

    /// The multiplyWithVectorFromLeft method takes a Vector as an input and creates a result.
    /// Then, multiplies values of input Vector starting from the left side with the values,
    /// accumulates the multiplication, and assigns to the result. If the sizes of both Vector
    /// and row number do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector type input
    ///
    /// returns: Vector that holds the result.
    ///
    pub fn multiply_with_vector_from_left(&mut self, vector: &Vector) -> Vector {
        if self.row != vector.size(){
            panic!("Number of rows should match the size of the vector");
        }
        let mut result: Vec<f64> = Vec::new();
        for i in 0..self.col {
            let mut sum = 0.0;
            for j in 0..self.row {
                sum += vector.get_value(j) * self.get_value(j, i);
            }
            result.push(sum);
        }
        Vector::new(result)
    }

    /// The multiplyWithVectorFromRight method takes a Vector as an input and creates a result.
    /// Then, multiplies values of input Vector starting from the right side with the values,
    /// accumulates the multiplication, and assigns to the result. If the sizes of both Vector
    /// and row number do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector type input.
    ///
    /// returns: Vector that holds the result.
    ///
    pub fn multiply_with_vector_from_right(&mut self, vector: &Vector) -> Vector {
        if self.col != vector.size(){
            panic!("Number of columns should match the size of the vector");
        }
        let mut result: Vec<f64> = Vec::new();
        for i in 0..self.row {
            let mut sum = 0.0;
            for j in 0..self.col {
                sum += vector.get_value(j) * self.get_value(i, j);
            }
            result.push(sum);
        }
        Vector::new(result)
    }

    /// The columnSum method takes a column number as an input and accumulates items at given column number of values
    ///
    /// # Arguments
    ///
    /// * `col`: Column number input.
    ///
    /// returns: f64 summation of given column of values
    ///
    pub fn column_sum(&self, col: usize) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.row {
            sum += self.get_value(i, col);
        }
        sum
    }

    /// The sumOfRows method creates a mew result Vector and adds the result of columnDum method's corresponding
    /// index to the newly created result.
    ///
    /// returns: Vector that holds column sum.
    ///
    pub fn sum_of_rows(&self) -> Vector {
        let mut result = Vector::initialize_same(0, 0.0);
        for i in 0..self.col {
            result.add(self.column_sum(i));
        }
        result
    }

    /// The rowSum method takes a row number as an input and accumulates items at given row number of values
    ///
    /// # Arguments
    ///
    /// * `row`: Row number input.
    ///
    /// returns: f64 summation of given row of values
    ///
    pub fn row_sum(&self, row: usize) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.col {
            sum += self.get_value(row, i);
        }
        sum
    }

    /// The multiply method takes a Matrix as an input. First it creates a result Matrix and puts the
    /// accumulated multiplication of values and given Matrix into result
    /// Matrix. If the size of Matrix's row size and values column size do not match,
    /// it panics.
    ///
    /// # Arguments
    ///
    /// * `m`: Matrix type input.
    ///
    /// returns: Matrix result
    ///
    pub fn multiply(&self, m: &Matrix) -> Matrix {
        if self.col != m.row {
            panic!("Matrix does not support multiplying");
        }
        let mut result = Matrix::new(self.row, m.col);
        for i in 0..self.row {
            for j in 0..m.col {
                let mut sum = 0.0;
                for k in 0..self.col {
                    sum += self.get_value(i, k) * m.get_value(k, j);
                }
                result.set_value(i, j, sum);
            }
        }
        result
    }

    /// The elementProduct method takes a Matrix as an input and performs element wise multiplication. Puts result
    /// to the newly created Matrix. If the size of Matrix's row and column size does not match with the values
    /// row and column size, it panics.
    ///
    /// # Arguments
    ///
    /// * `m`: Matrix type input.
    ///
    /// returns: Matrix result
    ///
    pub fn element_product(&self, m: &Matrix) -> Matrix {
        if self.row != m.row || self.col != m.col {
            panic!("Matrix does not support multiplying");
        }
        let mut result = Matrix::new(self.row, self.col);
        for i in 0..self.row {
            for j in 0..self.col {
                result.set_value(i, j, self.get_value(i, j) * m.get_value(i, j));
            }
        }
        result
    }

    /// The elementProduct method takes a Vector as an input and performs element wise multiplication. Puts result
    /// to the newly created Matrix.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector type input.
    ///
    /// returns: Matrix result
    ///
    pub fn element_product_with_vector(&mut self, vector: &Vector) -> Matrix {
        let mut result = Matrix::new(self.row, self.col);
        if self.row == 1 && self.col == vector.size(){
            for i in 0..self.col {
                result.set_value(0, i, self.get_value(0, i) * vector.get_value(i));
            }
        } else {
            if self.col == 1 && self.row == vector.size(){
                for i in 0..self.row {
                    result.set_value(i, 0, self.get_value(i, 0) * vector.get_value(i));
                }
            }
        }
        result
    }

    /// The sumOfElements method accumulates all the items in values {@link java.lang.reflect.Array} and
    /// returns this summation.
    ///
    /// returns: f64 sum of the items of values
    ///
    pub fn sum_of_elements(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.row {
            for j in 0..self.col {
                sum += self.get_value(i, j);
            }
        }
        sum
    }

    /// The trace method accumulates items of values at the diagonal.
    ///
    /// returns: f64 sum of items at diagonal.
    ///
    pub fn trace(&self) -> f64 {
        if self.row != self.col {
            panic!("Matrix should be square");
        }
        let mut sum = 0.0;
        for i in 0..self.row {
            sum += self.get_value(i, i);
        }
        sum
    }

    /// The transpose method creates a new Matrix, then takes the transpose of values
    /// and puts transposition to the Matrix.
    ///
    /// returns: Matrix type output.
    ///
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::new(self.col, self.row);
        for i in 0..self.row {
            for j in 0..self.col {
                result.set_value(j, i, self.get_value(i, j));
            }
        }
        result
    }

    /// The partial method takes 4 integer inputs; rowStart, rowEnd, colStart, colEnd and creates a Matrix size of
    /// rowEnd - rowStart + 1 x colEnd - colStart + 1. Then, puts corresponding items of values
    /// to the new result Matrix.
    ///
    /// # Arguments
    ///
    /// * `row_start`: integer input for defining starting index of row.
    /// * `row_end`: integer input for defining ending index of row.
    /// * `col_start`: integer input for defining starting index of column.
    /// * `col_end`: integer input for defining ending index of column.
    ///
    /// returns: Matrix result
    ///
    pub fn partial(&self, row_start: usize, row_end: usize, col_start: usize, col_end: usize) -> Matrix {
        let mut result = Matrix::new(row_end - row_start + 1, col_end - col_start + 1);
        for i in row_start..=row_end {
            for j in col_start..=col_end {
                result.set_value(i - row_start, j - col_start, self.get_value(i, j));
            }
        }
        result
    }

    /// The isSymmetric method compares each item of values {@link java.lang.reflect.Array} at positions (i, j) with (j, i)
    /// and returns true if they are equal, false otherwise.
    ///
    /// returns: true if items are equal, false otherwise.
    ///
    pub fn is_symmetric(&self) -> bool {
        if self.row != self.col {
            panic!("Matrix should be square");
        }
        for i in 0..self.row - 1 {
            for j in i + 1..self.row {
                if self.get_value(i, j) != self.get_value(j, i) {
                    return false;
                }
            }
        }
        true
    }

    /// The determinant method first creates a new {@link java.lang.reflect.Array}, and copies the items of  values
    /// into new. Then, calculates the determinant of this new.
    ///
    /// returns: f64 determinant of values
    ///
    pub fn determinant(&self) -> f64 {
        if self.row != self.col {
            panic!("Matrix should be square");
        }
        let mut det = 1.0;
        let mut copy = self.clone();
        for i in 0..self.row {
            det *= copy.get_value(i, i);
            if det == 0.0 {
                break;
            }
            for j in i + 1..self.row {
                let ratio = copy.get_value(j, i) / copy.get_value(i, i);
                for k in i..self.col {
                    copy.set_value(j, k, copy.get_value(j, k) - copy.get_value(i, k) * ratio);
                }
            }
        }
        det
    }

    fn swap_cells(&mut self, i: usize, j: usize, k: usize, l: usize) {
        let a = self.get_value(i, j);
        self.set_value(i, j, self.get_value(k, l));
        self.set_value(k, l, a);
    }

    /// The inverse method finds the inverse of values.
    ///
    pub fn inverse(&mut self) -> () {
        if self.row != self.col {
            panic!("Matrix should be square");
        }
        let mut b = Matrix::new(self.row, self.row);
        let mut indxc: Vec<usize> = Vec::new();
        let mut indxr: Vec<usize> = Vec::new();
        let mut ipiv: Vec<usize> = Vec::new();
        for _ in 0..self.row {
            ipiv.push(0);
        }
        for _ in 1..=self.row {
            let mut big = 0.0;
            let mut irow = self.row + 1;
            let mut icol = self.row + 1;
            for j in 1..=self.row {
                if ipiv[j - 1] != 1 {
                    for k in 1..=self.row{
                        if ipiv[k - 1] == 0 {
                            if self.get_value(j - 1, k - 1).abs() >= big {
                                big = self.get_value(j - 1, k - 1).abs();
                                irow = j;
                                icol = k;
                            }
                        }
                    }
                }
            }
            if irow == self.row + 1 || icol == self.row + 1 {
                panic!("Matrix is not invertible");
            }
            ipiv[icol - 1] += 1;
            if irow != icol {
                for l in 1..=self.row{
                    self.swap_cells(irow - 1, l - 1, icol - 1, l - 1)
                }
                for l in 1..=self.row{
                    let dum = b.values[(irow - 1) * self.row + l - 1];
                    b.values[(irow - 1) * self.row + l - 1] = b.values[(icol - 1) * self.row + l - 1];
                    b.values[(icol - 1) * self.row + l - 1] = dum;
                }
            }
            indxr.push(irow);
            indxc.push(icol);
            if self.get_value(icol - 1, icol - 1) == 0.0{
                panic!("Matrix is not invertible");
            }
            let pivinv = 1.0 / self.get_value(icol - 1, icol - 1);
            self.set_value(icol - 1, icol - 1, 1.0);
            for l in 1..=self.row{
                self.set_value(icol - 1, l - 1, self.get_value(icol - 1, l - 1) * pivinv);
            }
            for l in 1..=self.row{
                b.set_value(icol - 1, l - 1, b.get_value(icol - 1, l - 1) * pivinv);
            }
            for ll in 1..=self.row{
                if ll != icol{
                    let dum = self.get_value(ll - 1, icol - 1);
                    self.set_value(ll - 1, icol - 1, 0.0);
                    for l in 1..=self.row{
                        self.add_value(ll - 1, l - 1, -self.get_value(icol - 1, l - 1) * dum);
                    }
                    for l in 1..=self.row{
                        b.add_value(ll - 1, l - 1, -b.get_value(icol - 1, l - 1) * dum);
                    }
                }
            }
        }
        for l in 1..=self.row{
            if indxr[l - 1] != indxc[l - 1] {
                for k in 1..=self.row{
                    self.swap_cells(k - 1, indxr[l - 1] - 1, k - 1, indxc[l - 1] - 1)
                }
            }
        }
    }

    /// The choleskyDecomposition method creates a new Matrix and puts the Cholesky Decomposition of values Array
    /// into this Matrix. Also, it panics if it is not symmetric
    ///
    pub fn cholesky_decomposition(&self) -> Matrix {
        if !self.is_symmetric() {
            panic!("Matrix should be symmetric");
        }
        let mut b = Matrix::new(self.row, self.col);
        for i in 0..self.row {
            for j in i..self.row {
                let mut sum = self.get_value(i, j);
                for k in 0..i{
                    sum -= self.get_value(i, k) * self.get_value(j, k);
                }
                if i == j{
                    if sum <= 0.0{
                        panic!("Matrix not positive definite");
                    }
                    b.set_value(i, i, sum.sqrt())
                } else {
                    b.set_value(i, j, sum / b.get_value(i, i))
                }
            }
        }
        b
    }

    /// The rotate method rotates matrix according to given inputs.
    ///
    /// # Arguments
    ///
    /// * `s`: double input.
    /// * `tau`: double input.
    /// * `i`: integer input.
    /// * `j`: integer input.
    /// * `k`: integer input.
    /// * `l`: integer input.
    ///
    fn rotate(&mut self, s: f64, tau: f64, i: usize, j: usize, k: usize, l: usize){
        let g = self.get_value(i, j);
        let h = self.get_value(k, l);
        self.set_value(i, j, g - s * (h + g * tau));
        self.set_value(k, l, h + s * (g - h * tau));
    }

    /// The characteristics method finds and returns a sorted vector of Eigenvectors. And it panics
    /// if it is not symmetric.
    ///
    /// returns: Vec<Eigenvector> a sorted vector of Eigenvectors.
    ///
    pub fn characteristics(&self) -> Vec<Eigenvector> {
        if !self.is_symmetric() {
            panic!("Matrix should be symmetric");
        }
        let mut matrix1 = self.clone();
        let mut v = Matrix::identity(matrix1.row);
        let mut d : Vec<f64> = Vec::new();
        let mut b : Vec<f64> = Vec::new();
        let mut z : Vec<f64> = Vec::new();
        let eps = 0.000000000000000001;
        let mut result: Vec<Eigenvector> = Vec::new();
        for ip in 0..self.row{
            b.push(matrix1.get_value(ip, ip));
            d.push(matrix1.get_value(ip, ip));
            z.push(0.0);
        }
        for i in 1..=50{
            let mut sm = 0.0;
            for ip in 0..self.row - 1{
                for iq in ip + 1..self.row{
                    sm += matrix1.get_value(ip, iq).abs();
                }
            }
            if sm == 0.0{
                break;
            }
            let threshold : f64;
            if i < 4{
                threshold = 0.2 * sm / (self.row * self.row) as f64;
            } else {
                threshold = 0.0;
            }
            for ip in 0..self.row - 1{
                for iq in ip + 1..self.row{
                    let g = 100.0 * matrix1.get_value(ip, iq).abs();
                    if i > 4 && g <= eps * d[ip].abs() && g <= eps * d[iq].abs(){
                        matrix1.set_value(ip, iq, 0.0);
                    } else {
                        if matrix1.get_value(ip, iq).abs() > threshold{
                            let mut h = d[iq] - d[ip];
                            let mut t : f64;
                            if g <= eps * h.abs(){
                                t = matrix1.get_value(ip, iq) / h;
                            } else {
                                let theta = 0.5 * h / matrix1.get_value(ip, iq);
                                t = 1.0 / (theta.abs() + (1.0 + theta * theta).sqrt());
                                if theta < 0.0{
                                    t = -t;
                                }
                            }
                            let c = 1.0 / (1.0 + t * t).sqrt();
                            let s = t * c;
                            let tau = s / (1.0 + c);
                            h = t * matrix1.get_value(ip, iq);
                            z[ip] -= h;
                            z[iq] += h;
                            d[ip] -= h;
                            d[iq] += h;
                            matrix1.set_value(ip, iq, 0.0);
                            for j in 0..ip{
                                matrix1.rotate(s, tau, j, ip, j, iq);
                            }
                            for j in ip + 1..iq{
                                matrix1.rotate(s, tau, ip, j, j, iq);
                            }
                            for j in iq + 1..self.row{
                                matrix1.rotate(s, tau, ip, j, iq, j);
                            }
                            for j in 0..self.row{
                                v.rotate(s, tau, j, ip, j, iq);
                            }
                        }
                    }
                }
            }
            for ip in 0..self.row{
                b[ip] += z[ip];
                d[ip] = b[ip];
                z[ip] = 0.0
            }
        }
        for i in 0..self.row{
            if d[i] > 0.0{
                result.push(Eigenvector::new(d[i], v.get_column_vector(i)))
            }
        }
        result.sort_by(|a, b| b.eigen_value.partial_cmp(&a.eigen_value).unwrap_or(std::cmp::Ordering::Equal));
        result
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn test_column_wise_normalize() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let mut identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        small.columnwise_normalize();
        assert_relative_eq!(small.sum_of_elements(), 3.0, epsilon = 0.000001);
        identity.columnwise_normalize();
        assert_relative_eq!(identity.sum_of_elements(), 100.0, epsilon = 0.000001);
        large.columnwise_normalize();
        assert_relative_eq!(large.sum_of_elements(), 1000.0, epsilon = 0.001);
    }

    #[test]
    fn test_multiply_by_constant() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        small.multiply_with_constant(4.0);
        assert_relative_eq!(small.sum_of_elements(), 36.0, epsilon = 0.1);
        large.multiply_with_constant(1.001);
        assert_relative_eq!(large.sum_of_elements(), 1001000.0, epsilon = 0.1);
    }

    #[test]
    fn test_divide_by_constant() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        small.divide_by_constant(4.0);
        assert_relative_eq!(small.sum_of_elements(), 2.25, epsilon = 0.0);
        large.divide_by_constant(10.0);
        assert_relative_eq!(large.sum_of_elements(), 100000.0, epsilon = 0.0001);
    }

    #[test]
    fn test_add() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let mut small2 = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small2.set_value(i, j, (i + j) as f64);
            }
        }
        small.add(&small2);
        assert_eq!(small.sum_of_elements(), 27.0);
    }

    #[test]
    fn test_add_vector() {
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        let large_v = Vector::initialize_same(1000, 1.0);
        large.add_vector(4, &large_v);
        assert_eq!(large.sum_of_elements(), 1001000.0);
    }

    #[test]
    fn test_subtract() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let mut small2 = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small2.set_value(i, j, (i + j) as f64);
            }
        }
        small.subtract(&small2);
        assert_eq!(small.sum_of_elements(), -9.0);
    }

    #[test]
    fn test_multiply_with_vector_from_left() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let v = Vector::initialize_same(3, 1.0);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        let large_v = Vector::initialize_same(1000, 1.0);
        let result1 =small.multiply_with_vector_from_left(&v);
        let result2 =large.multiply_with_vector_from_left(&large_v);
        assert_eq!(result1.sum_of_elements(), 9.0);
        assert_eq!(result2.sum_of_elements(), 1000000.0);
    }

    #[test]
    fn test_multiply_with_vector_from_right() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let v = Vector::initialize_same(3, 1.0);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        let large_v = Vector::initialize_same(1000, 1.0);
        let result1 =small.multiply_with_vector_from_right(&v);
        let result2 =large.multiply_with_vector_from_right(&large_v);
        assert_eq!(result1.sum_of_elements(), 9.0);
        assert_eq!(result2.sum_of_elements(), 1000000.0);
    }

    #[test]
    fn test_column_sum() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        assert_eq!(small.column_sum(0), 3.0);
        assert_eq!(identity.column_sum(0), 1.0);
        assert_eq!(large.column_sum(0), 1000.0);
    }

    #[test]
    fn test_sum_of_rows() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        assert_eq!(small.sum_of_rows().sum_of_elements(), 9.0);
        assert_eq!(identity.sum_of_rows().sum_of_elements(), 100.0);
        assert_eq!(large.sum_of_rows().sum_of_elements(), 1000000.0);
    }

    #[test]
    fn test_row_sum() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        assert_eq!(small.row_sum(0), 3.0);
        assert_eq!(identity.row_sum(0), 1.0);
        assert_eq!(large.row_sum(0), 1000.0);
    }

    #[test]
    fn test_multiply() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        let result1 = small.multiply(&small);
        assert_eq!(result1.sum_of_elements(), 27.0);
        let result2 = identity.multiply(&identity);
        assert_eq!(result2.sum_of_elements(), 100.0);
        let result3 = large.multiply(&large);
        assert_eq!(result3.sum_of_elements(), 1000000000.0);
    }

    #[test]
    fn test_element_product() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        let result1 = small.element_product(&small);
        assert_eq!(result1.sum_of_elements(), 9.0);
        let result2 = identity.element_product(&identity);
        assert_eq!(result2.sum_of_elements(), 100.0);
        let result3 = large.element_product(&large);
        assert_eq!(result3.sum_of_elements(), 1000000.0);
    }

    #[test]
    fn test_trace() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        assert_eq!(small.trace(), 3.0);
        assert_eq!(identity.trace(), 100.0);
        assert_eq!(large.trace(), 1000.0);
    }

    #[test]
    fn test_transpose() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        assert_relative_eq!(small.transpose().sum_of_elements(), 9.0, epsilon = 0.0);
        assert_relative_eq!(identity.transpose().sum_of_elements(), 100.0, epsilon = 0.0);
        assert_relative_eq!(large.transpose().sum_of_elements(), 1000000.0, epsilon = 0.0);
    }

    #[test]
    fn test_symmetric() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        assert!(small.is_symmetric());
        assert!(identity.is_symmetric());
        assert!(large.is_symmetric());
    }

    #[test]
    fn test_determinant() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let identity = Matrix::identity(100);
        let mut large = Matrix::new(1000,1000);
        for i in 0..1000{
            for j in 0..1000{
                large.set_value(i, j, 1.0);
            }
        }
        assert_eq!(small.determinant(), 0.0);
        assert_eq!(identity.determinant(), 1.0);
        assert_eq!(large.determinant(), 0.0);
    }

    #[test]
    fn test_inverse() {
        let mut identity = Matrix::identity(100);
        identity.inverse();
        assert_eq!(identity.sum_of_elements(), 100.0);
    }

    #[test]
    fn test_characteristics() {
        let mut small = Matrix::new(3,3);
        for i in 0..3{
            for j in 0..3{
                small.set_value(i, j, 1.0);
            }
        }
        let vectors1 = small.characteristics();
        assert_eq!(vectors1.len(), 2);
        let identity = Matrix::identity(100);
        let vectors2 = identity.characteristics();
        assert_eq!(vectors2.len(), 100);
    }


}