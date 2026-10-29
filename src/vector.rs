pub struct Vector {
    size: usize,
    values: Vec<f64>,
}

impl Vector {
    pub fn new(values: Vec<f64>) -> Vector {
        Self{size:values.len(), values }
    }

    /// Another constructor of Vector class which takes integer size and double x as inputs. Then, initializes size
    /// variable with given size input and creates new values and adds given input x to values.
    ///
    /// # Arguments
    ///
    /// * `size`: vector size
    /// * `x`: item to add values
    ///
    /// returns: Vector
    ///
    pub fn initialize_same(size:usize, x: f64) -> Vector {
        Vector{size, values:vec![x; size]}
    }

    /// Another constructor of Vector class which takes integer size, integer index and double x as inputs. Then, initializes size
    /// variable with given size input and creates new values and adds 0.0 to values.
    /// Then, sets the item of values at given index as given input x.
    ///
    /// # Arguments
    ///
    /// * `size`: size of the vector
    /// * `index`: to set a particular item.
    /// * `x`: item to add values' given index.
    ///
    /// returns: Vector
    ///
    pub fn one_hot_vector(size: usize, index: usize, x: f64) -> Vector {
        let mut result = Vector{size, values:vec![x; size]};
        result.values[index] = x;
        result
    }

    /// The biased method creates a Vector result, add adds each item of values into the result Vector.
    /// Then, insert 1.0 to 0th position and return result Vector.
    ///
    /// returns: Vector
    ///
    pub fn biased(&self) -> Vector {
        let mut result = Vector{size:self.size + 1, values:self.values.clone()};
        result.values.insert(0, 1.0);
        result
    }

    /// The add method adds given input to the values and increments the size variable by one.
    ///
    /// # Arguments
    ///
    /// * `x`: double input to add values
    ///
    pub fn add(&mut self, x: f64) {
        self.values.push(x);
        self.size += 1;
    }

    /// The insert method puts given input to the given index of values and increments the size variable by one.
    ///
    /// # Arguments
    ///
    /// * `pos`: index to insert input.
    /// * `x`: input to insert to given index of values
    ///
    pub fn insert(&mut self, pos: usize, x: f64) {
        self.values.insert(pos, x);
        self.size += 1;
    }

    /// The remove method deletes the item at given input position of values and decrements the size variable by one.
    ///
    /// # Arguments
    ///
    /// * `pos`: index to remove from values
    ///
    pub fn remove(&mut self, pos: usize) {
        self.values.remove(pos);
        self.size -= 1;
    }

    /// The clear method sets all the elements of values to 0.0.
    ///
    pub fn clear(&mut self) {
        for i in 0..self.size {
            self.values[i] = 0.0;
        }
    }

    /// The sumOfElements method sums up all elements in the vector.
    ///
    /// returns: f64 Sum of all elements in the vector.
    ///
    pub fn sum_of_elements(&self) -> f64 {
        self.values.iter().sum()
    }

    /// The maxIndex method gets the first item of values as maximum item, then it loops through the indices
    /// and if a greater value than the current maximum item comes, it updates the maximum item and returns the final
    ///  maximum item's index.
    ///
    /// returns: usize final maximum item's index.
    ///
    pub fn max_index(&self) -> usize {
        let mut index:usize = 0;
        let mut max: f64 = self.values[0];
        for i in 1..self.size {
            if self.values[i] > max {
                max = self.values[i];
                index = i;
            }
        }
        index
    }

    /// The sigmoid method loops through the values and sets each ith item with sigmoid function, i.e
    /// 1 / (1 + Math.exp(-values.get(i))), i ranges from 0 to size.
    pub fn sigmoid(&mut self) -> () {
        for i in 0..self.size {
            self.values[i] = 1.0 / (1.0 + (-self.values[i]).exp());
        }
    }

    /// The tanh method loops through the values and sets each ith item with tanh function.
    pub fn tanh(&mut self) -> () {
        for i in 0..self.size {
            self.values[i] = self.values[i].tanh();
        }
    }

    /// The relu method loops through the values and sets each ith item with relu function.
    pub fn relu(&mut self) -> () {
        for i in 0..self.size {
            if self.values[i] < 0.0 {
                self.values[i] = 0.0;
            }
        }
    }

    /// The reluDerivative method loops through the values and sets each ith item with the derivative of
    /// relu function.
    pub fn relu_derivative(&mut self) -> () {
        for i in 0..self.size {
            if self.values[i] < 0.0 {
                self.values[i] = 0.0;
            } else {
                self.values[i] = 1.0;
            }
        }
    }

    /// The skipVector method takes a mod and a value as inputs. It creates a new result Vector, and assigns given input value to i.
    /// While i is less than the size, it adds the ith item of values to the result and increments i by given mod input.
    ///
    /// # Arguments
    ///
    /// * `modulus`: integer input.
    /// * `value`: integer input.
    ///
    /// returns: Vector result Vector.
    ///
    pub fn skip_vector(&mut self, modulus: usize, value: usize) -> Vector {
        let mut result = Vector::initialize_same(0, 0.0);
        let mut i = value;
        while i < self.size {
            result.add(self.values[i]);
            i += modulus;
        }
        result
    }

    /// The add method takes a Vector v as an input. It sums up the corresponding elements of both given vector's
    /// alues and values  and puts result back to the values.
    /// If their sizes do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector to add.
    ///
    pub fn add_vector(&mut self, vector: &Vector) -> () {
        if vector.size != self.size {
            panic!("vectors don't match");
        }
        for i in 0..self.size {
            self.values[i] += vector.values[i];
        }
    }

    /// The subtract method takes a Vector v as an input. It subtracts the corresponding elements of given vector's
    /// values from values and puts result back to the values.
    /// If their sizes do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector to add.
    ///
    pub fn subtract_vector(&mut self, vector: &Vector) -> () {
        if vector.size != self.size {
            panic!("vectors don't match");
        }
        for i in 0..self.size {
            self.values[i] -= vector.values[i];
        }
    }

    /// The difference method takes a Vector v as an input. It creates a new result, then subtracts the corresponding elements of given vector's
    /// values from values and puts result back to the result.
    /// If their sizes do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector to find difference from values
    ///
    /// returns: Vector new Vector with result
    ///
    pub fn difference(&mut self, vector: &Vector) -> Vector {
        if vector.size != self.size {
            panic!("vectors don't match");
        }
        let mut result = Vec::new();
        for i in 0..self.size {
            result.push(self.values[i] - vector.values[i]);
        }
        Vector{size:result.len(), values:result}
    }

    /// The dotProduct method takes a Vector vector as an input. It creates a new double variable result, then
    /// multiplies the corresponding elements of given vector's values with values and assigns
    /// the multiplication to the result. If their sizes do not match, it throws a VectorSizeMismatch exception.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector to find dot product.
    ///
    /// returns: f64 double result.
    ///
    pub fn dot_product(&self, vector: &Vector) -> f64 {
        if vector.size != self.size {
            panic!("vectors don't match");
        }
        let mut sum = 0.0;
        for i in 0..self.size {
            sum += self.values[i] * vector.values[i];
        }
        sum
    }

    /// The dotProduct method creates a new double variable result, then squares the elements of values and assigns
    /// the accumulation to the result.
    ///
    /// returns: f64 double result.
    ///
    pub fn dot_product_itself(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.size {
            sum += self.values[i] * self.values[i];
        }
        sum
    }

    /// The elementProduct method takes a Vector v as an input. It creates a new double result, then
    /// multiplies the corresponding elements of given vector's values with values and assigns
    /// the multiplication to the result. If their sizes do not match, it panics.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector to find element product.
    ///
    /// returns: Vector Vector with result
    ///
    pub fn element_product(&mut self, vector: &Vector) -> Vector {
        if vector.size != self.size {
            panic!("vectors don't match");
        }
        let mut result = Vec::new();
        for i in 0..self.size {
            result.push(self.values[i] * vector.values[i]);
        }
        Vector{size:result.len(), values:result}
    }

    /// The divide method takes a double value as an input and divides each item of values with given value.
    ///
    /// # Arguments
    ///
    /// * `value`: is used to divide items of values
    ///
    pub fn divide(&mut self, value: f64) -> () {
        for i in 0..self.size {
            self.values[i] /= value;
        }
    }

    /// The multiply method takes a double value as an input and multiplies each item of values with given value.
    ///
    /// # Arguments
    ///
    /// * `value`: is used to multiply items of values
    ///
    pub fn multiply(&mut self, value: f64) -> () {
        for i in 0..self.size {
            self.values[i] *= value;
        }
    }

    /// The product method takes a double value as an input and creates a new result, then multiplies each
    /// item of values with given value and adds to the result.
    ///
    /// # Arguments
    ///
    /// * `value`: is used to multiply items of values
    ///
    /// returns: Vector result
    ///
    pub fn product(self, value: f64) -> Vector {
        let mut result = Vector::initialize_same(0, 0.0);
        for i in 0..self.size {
            result.add(self.values[i] * value);
        }
        result
    }

    /// The l1Normalize method is used to apply Least Absolute Errors, it accumulates items of values and sets
    /// each item by dividing it by the summation value.
    pub fn l1_norm(&mut self) {
        let sum = self.sum_of_elements();
        for i in 0..self.size {
            self.values[i] /= sum;
        }
    }

    /// The l2Norm method is used to apply Least Squares, it accumulates second power of each items of values
    /// and returns the square root of this summation.
    ///
    /// returns: f64 square root of this summation.
    ///
    pub fn l2_norm(&self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.size {
            sum += self.values[i] * self.values[i];
        }
        sum.sqrt()
    }

    /// The cosineSimilarity method takes a Vector vector as an input and returns the result of dotProduct(v) / l2Norm() / v.l2Norm().
    /// If sizes do not match it panics.
    ///
    /// # Arguments
    ///
    /// * `vector`: Vector input.
    ///
    /// returns: f64 dotProduct(v) / l2Norm() / v.l2Norm().
    ///
    pub fn cosine_similarity(&self, vector: &Vector) -> f64 {
        if vector.size != self.size {
            panic!("vectors don't match");
        }
        self.dot_product(vector) / self.l2_norm() / vector.l2_norm()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    /// Getter for the item at given index of values
    ///
    /// # Arguments
    ///
    /// * `index`: used to get an item.
    ///
    /// returns: f64 the item at given index.
    ///
    pub fn get_value(&self, index: usize) -> f64 {
        self.values[index]
    }

    /// Setter for the setting the value at given index of values
    ///
    /// # Arguments
    ///
    /// * `index`: to set.
    /// * `value`: is used to set the given index
    ///
    pub fn set_value(&mut self, index: usize, value: f64) -> () {
        self.values[index] = value;
    }

    /// The addValue method adds the given value to the item at given index of values.
    ///
    /// # Arguments
    ///
    /// * `index`: to add the given value.
    /// * `value`: value to add to given index.
    ///
    pub fn add_value(&mut self, index: usize, value: f64) -> () {
        self.values[index] += value;
    }

}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use crate::vector::Vector;

    #[test]
    fn test_biased() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let biased = small_vector_1.biased();
        assert_eq!(1.0, biased.get_value(0));
        assert_eq!(small_vector_1.size() + 1, biased.size());
    }

    #[test]
    fn test_element_add() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        small_vector_1.add(7.0);
        assert_eq!(7.0, small_vector_1.get_value(5));
        assert_eq!(6, small_vector_1.size());
    }

    #[test]
    fn test_insert() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        small_vector_1.insert(3, 6.0);
        assert_eq!(6.0, small_vector_1.get_value(3));
        assert_eq!(6, small_vector_1.size());
    }

    #[test]
    fn test_remove() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        small_vector_1.remove(2);
        assert_eq!(5.0, small_vector_1.get_value(2));
        assert_eq!(4, small_vector_1.size());
    }

    #[test]
    fn test_sum_of_elements() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        assert_eq!(20.0, small_vector_1.sum_of_elements());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        assert_eq!(30.0, small_vector_2.sum_of_elements());
    }

    #[test]
    fn test_max_index() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        assert_eq!(4, small_vector_1.max_index());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        assert_eq!(0, small_vector_2.max_index());
    }

    #[test]
    fn test_sigmoid() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        small_vector_1.sigmoid();
        assert_relative_eq!(0.8807971, small_vector_1.get_value(0), epsilon = 0.000001);
        assert_relative_eq!(0.9975274, small_vector_1.get_value(4), epsilon = 0.000001);
    }

    #[test]
    fn test_skip_vector() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let mut small_vector_3 = small_vector_1.skip_vector(2, 0);
        assert_eq!(2.0, small_vector_3.get_value(0));
        assert_eq!(6.0, small_vector_3.get_value(2));
        small_vector_3 = small_vector_1.skip_vector(3, 1);
        assert_eq!(3.0, small_vector_3.get_value(0));
        assert_eq!(6.0, small_vector_3.get_value(1));
    }

    #[test]
    fn test_vector_add() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        small_vector_1.add_vector(&small_vector_2);
        assert_eq!(50.0, small_vector_1.sum_of_elements());
    }

    #[test]
    fn test_vector_subtract() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        small_vector_1.subtract_vector(&small_vector_2);
        assert_eq!(-10.0, small_vector_1.sum_of_elements());
    }

    #[test]
    fn test_vector_difference() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        let small_vector_3 = small_vector_1.difference(&small_vector_2);
        assert_eq!(-10.0, small_vector_3.sum_of_elements());
    }

    #[test]
    fn test_dot_product() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        let dot_product = small_vector_1.dot_product(&small_vector_2);
        assert_eq!(110.0, dot_product);
    }

    #[test]
    fn test_dot_product_with_itself() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let dot_product = small_vector_1.dot_product_itself();
        assert_eq!(90.0, dot_product);
    }

    #[test]
    fn test_element_product() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        let small_vector_3 = small_vector_1.element_product(&small_vector_2);
        assert_eq!(110.0, small_vector_3.sum_of_elements());
    }

    #[test]
    fn test_divide() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        small_vector_1.divide(10.0);
        assert_eq!(2.0, small_vector_1.sum_of_elements());
    }

    #[test]
    fn test_multiply() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        small_vector_1.multiply(10.0);
        assert_eq!(200.0, small_vector_1.sum_of_elements());
    }

    #[test]
    fn test_product() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let small_vector_3 = small_vector_1.product(7.0);
        assert_eq!(140.0, small_vector_3.sum_of_elements());
    }

    #[test]
    fn test_l1_normalize() {
        let mut small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        small_vector_1.l1_norm();
        assert_eq!(1.0, small_vector_1.sum_of_elements());
    }

    #[test]
    fn test_l2_normalize() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let norm = small_vector_1.l2_norm();
        assert_eq!(90.0f64.sqrt(), norm);
    }

    #[test]
    fn test_cosine_similarity() {
        let small_vector_1 = Vector::new([2.0, 3.0, 4.0, 5.0, 6.0].to_vec());
        let small_vector_2 = Vector::new([8.0, 7.0, 6.0, 5.0, 4.0].to_vec());
        let similarity = small_vector_1.cosine_similarity(&small_vector_2);
        assert_relative_eq!(0.8411910, similarity, epsilon = 0.000001);
    }

}