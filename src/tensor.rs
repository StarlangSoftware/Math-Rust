use std::cmp::max;

pub struct Tensor{
    shape: Vec<usize>,
    strides: Vec<usize>,
    data: Vec<f64>
}

impl Tensor{

    /// Initializes the tensor directly with a primitive double array and shape.
    /// This is the fastest constructor used for internal mathematical operations.
    ///
    /// # Arguments
    ///
    /// * `data`: Primitive array representing the flattened tensor data.
    /// * `shape`: The shape of the tensor.
    ///
    /// returns: Tensor
    ///
    pub fn new(data: Vec<f64>, shape: Vec<usize>) -> Tensor{
        if Tensor::compute_number_of_elements(&shape) != data.len(){
            panic!("Shape does not match the number of elements in data.");
        }
        let strides = Self::compute_strides(&shape);
        Tensor{shape, strides, data}
    }

    /// Computes the strides for each dimension based on the shape.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape vector for the data
    ///
    /// returns: Vec<usize> Strides of the tensor
    ///
    fn compute_strides(shape: &Vec<usize>) -> Vec<usize>{
        let mut strides: Vec<usize> = vec![0; shape.len()];
        let mut product = 1;
        for i in (0..shape.len()).rev(){
            strides[i] = product;
            product *= shape[i];
        }
        strides
    }

    /// Concatenates two tensors into a one.
    ///
    /// # Arguments
    ///
    /// * `tensor`: 2nd tensor for concatenation.
    /// * `dimension`: to concatenate.
    ///
    /// returns: Tensor Concatenated Tensor.
    ///
    pub fn concat(&self, tensor: &Tensor, dimension: usize) -> Tensor{
        if dimension >= self.shape.len(){
            panic!("Dimension out of bounds.");
        }
        if self.shape.len() != tensor.shape.len(){
            panic!("Dimensions length do not match.");
        }
        let mut start_index = 1;
        let mut end_index1 = 1;
        let mut end_index2 = 1;
        for i in 0..self.shape.len(){
            if i != dimension && self.shape[i] != tensor.shape[i]{
                panic!("Dimensions does not match.");
            }
            if i >= dimension{
                end_index1 *= self.shape[i];
                end_index2 *= tensor.shape[i];
            } else {
                start_index *= self.shape[i];
            }
        }
        let mut new_shape: Vec<usize> = Vec::with_capacity(self.shape.len());
        for i in 0..self.shape.len(){
            if i == dimension{
                new_shape.push(self.shape[i] + tensor.shape[i]);
            } else {
                new_shape.push(self.shape[i]);
            }
        }
        let mut new_data: Vec<f64> = Vec::with_capacity(start_index * (end_index1 + end_index2));
        for i in 0..start_index{
            for j in 0..end_index1{
                new_data.push(self.data[i * end_index1 + j]);
            }
            for j in 0..end_index2{
                new_data.push(tensor.data[i * end_index2 + j]);
            }
        }
        Tensor::new(new_data, new_shape)
    }

    /// Returns the sub-Tensor taking the given dimensions.
    ///
    /// # Arguments
    ///
    /// * `dimensions`: Dimensions of the subTensor
    ///
    /// returns: Tensor a sub-Tensor.
    ///
    pub fn get(&self, dimensions: &Vec<usize>) -> Tensor{
        if dimensions.len() >= self.shape.len(){
            panic!("Dimensions exceeds or same as the tensor's dimension.");
        }
        for i in 0..self.shape.len(){
            if dimensions[i] >= self.shape[i]{
                panic!("There is a dimension length exceed the tensor's dimension length.");
            }
        }
        let new_shape: Vec<usize> = self.shape[dimensions.len()..self.shape.len()].to_vec();
        let mut i = 0;
        let mut start = 0;
        let mut end = self.data.len();
        while i < dimensions.len(){
            let parts = (end - start) / self.shape[i];
            start += parts * dimensions[i];
            end = start + parts;
            i = i + 1;
        }
        let new_data : Vec<f64> = self.data[start..end].to_vec();
        Tensor::new(new_data, new_shape)
    }

    /// Computes the total number of elements in the tensor based on its shape.
    ///
    /// # Arguments
    ///
    /// * `shape`: Shape of the tensor
    ///
    /// returns: i32 Size of the tensor
    ///
    fn compute_number_of_elements(shape: &Vec<usize>) -> usize{
        let mut product = 1;
        for i in 0..shape.len(){
            product *= shape[i];
        }
        product
    }

    /// Retrieves the value at the given indices.
    ///
    /// # Arguments
    ///
    /// * `indices`: Indices of the tensor
    ///
    /// returns: f64 Value at the given index
    ///
    pub fn get_value(&self, indices: &Vec<usize>) -> f64{
        let mut flat_index = 0;
        for i in 0..indices.len(){
            flat_index += indices[i] * self.strides[i];
        }
        self.data[flat_index]
    }

    /// Sets the value at the given indices.
    ///
    /// # Arguments
    ///
    /// * `indices`: Indices of the tensor
    /// * `value`: New value
    ///
    pub fn set(&mut self, indices: &Vec<usize>, value: f64){
        let mut flat_index = 0;
        for i in 0..indices.len(){
            flat_index += indices[i] * self.strides[i];
        }
        self.data[flat_index] = value;
    }

    /// Reshapes the tensor to the specified new shape. Memory is shared!
    ///
    /// # Arguments
    ///
    /// * `new_shape`: New shape of the tensor
    ///
    /// returns: Tensor Reshaped tensor
    ///
    pub fn reshape(&self, new_shape: Vec<usize>) -> Tensor{
        if Tensor::compute_number_of_elements(&new_shape) != Tensor::compute_number_of_elements(&self.shape) {
            panic!("Total number of elements must remain the same.");
        }
        Tensor::new(self.data.clone(), new_shape)
    }

    /// Transposes the tensor according to the specified axes.
    ///
    /// # Arguments
    ///
    /// * `axes`: Axes of the tensor
    ///
    /// returns: Tensor Transposed tensor
    ///
    pub fn transpose(&self, axes: Option<&Vec<usize>>) -> Tensor{
        let mut new_axes : Vec<usize>;
        if axes.is_none(){
            new_axes = Vec::with_capacity(self.shape.len());
            for i in 0..self.shape.len(){
                new_axes.push(self.shape.len() - 1 - i);
            }
        } else {
            new_axes = axes.unwrap().clone();
        }
        let mut new_shape: Vec<usize> = Vec::with_capacity(self.shape.len());
        for i in 0..new_axes.len(){
            new_shape.push(self.shape[new_axes[i]]);
        }
        let flattened_data = self.transpose_flattened_data(&new_axes, &new_shape);
        Tensor::new(flattened_data, new_shape)
    }

    /// Rearranges the flattened data for transposition.
    ///
    /// # Arguments
    ///
    /// * `axes`: New axes for the transposed vector
    /// * `new_shape`: New transposed shape
    ///
    /// returns: Vec<f64, Global> Transposed data
    ///
    fn transpose_flattened_data(&self, axes: &Vec<usize>, new_shape: &Vec<usize>) -> Vec<f64>{
        let new_strides = Tensor::compute_strides(new_shape);
        let num_elements = Tensor::compute_number_of_elements(&new_shape);
        let mut flattened_data: Vec<f64> = Vec::with_capacity(num_elements);
        for i in 0..num_elements{
            let new_indices = Tensor::unflatten_index(i, &new_strides);
            let mut original_indices: Vec<usize> = Vec::with_capacity(self.shape.len());
            for dim in 0..self.shape.len(){
                let mut original_dim_index = -1;
                for j in 0..axes.len(){
                    if axes[j] == dim {
                        original_dim_index = j as i32;
                        break;
                    }
                }
                if original_dim_index != -1{
                    original_indices.push(new_indices[original_dim_index as usize]);
                }
            }
            flattened_data.push(self.get_value(&original_indices));
        }
        flattened_data
    }

    fn unflatten_index(flat_index: usize, strides: &Vec<usize>) -> Vec<usize>{
        let mut new_index = flat_index;
        let mut indices : Vec<usize> = Vec::with_capacity(strides.len());
        for i in 0..strides.len(){
            indices.push(new_index / strides[i]);
            new_index = new_index % strides[i];
        }
        indices
    }

    fn broadcast_shape(&self, shape1: &Vec<usize>, shape2: &Vec<usize>) -> Vec<usize>{
        let reversed_shape1: Vec<usize> = shape1.iter().rev().cloned().collect();
        let reversed_shape2: Vec<usize> = shape2.iter().rev().cloned().collect();
        let mut result_shape : Vec<usize> = Vec::new();
        let max_length = max(shape1.len(), shape2.len());
        for i in 0..max_length{
            let dim1 = if i < reversed_shape1.len() {reversed_shape1[i]} else {1};
            let dim2 = if i < reversed_shape2.len() {reversed_shape2[i]} else {1};
            if dim1 == dim2 {
                result_shape.push(dim1);
            } else {
                if dim1 == 1 || dim2 == 1{
                    result_shape.push(max(dim1, dim2));
                }
            }
        }
        for i in reversed_shape2.len()..reversed_shape1.len(){
            result_shape.push(reversed_shape1[i]);
        }
        for i in reversed_shape1.len()..reversed_shape2.len(){
            result_shape.push(reversed_shape2[i]);
        }
        result_shape.iter().rev().cloned().collect()
    }

    pub fn broadcast_to(&self, target_shape: &Vec<usize>) -> Tensor{
        let mut expanded_shape: Vec<usize> = vec![1; target_shape.len() - self.shape.len()];
        expanded_shape.extend(self.shape.clone());
        let num_elements = Tensor::compute_number_of_elements(&target_shape);
        let mut new_data: Vec<f64> = Vec::with_capacity(num_elements);
        let target_strides = Tensor::compute_strides(&target_shape);
        for i in 0..num_elements{
            let indices = Tensor::unflatten_index(i, &target_strides);
            let mut original_indices : Vec<usize> = Vec::with_capacity(indices.len());
            for j in 0..indices.len(){
                original_indices.push(if expanded_shape[j] > 1 {indices[j]} else {0});
            }
            new_data.push(self.get_value(&original_indices));
        }
        Tensor::new(new_data, target_shape.clone())
    }

    pub fn add(&self, other: &Tensor) -> Tensor{
        let broadcase_shape = self.broadcast_shape(&self.shape, &other.shape);
        let tensor1 = self.broadcast_to(&broadcase_shape);
        let tensor2 = other.broadcast_to(&broadcase_shape);
        let num_elements = Tensor::compute_number_of_elements(&broadcase_shape);
        let mut result_data : Vec<f64> = Vec::with_capacity(num_elements);
        for i in 0..num_elements{
            result_data.push(tensor1.data[i] + tensor2.data[i]);
        }
        Tensor::new(result_data, broadcase_shape)
    }

    pub fn subtract(&self, other: &Tensor) -> Tensor{
        let broadcase_shape = self.broadcast_shape(&self.shape, &other.shape);
        let tensor1 = self.broadcast_to(&broadcase_shape);
        let tensor2 = other.broadcast_to(&broadcase_shape);
        let num_elements = Tensor::compute_number_of_elements(&broadcase_shape);
        let mut result_data : Vec<f64> = Vec::with_capacity(num_elements);
        for i in 0..num_elements{
            result_data.push(tensor1.data[i] - tensor2.data[i]);
        }
        Tensor::new(result_data, broadcase_shape)
    }

    pub fn hadamard_product(&self, other: &Tensor) -> Tensor{
        let broadcase_shape = self.broadcast_shape(&self.shape, &other.shape);
        let tensor1 = self.broadcast_to(&broadcase_shape);
        let tensor2 = other.broadcast_to(&broadcase_shape);
        let num_elements = Tensor::compute_number_of_elements(&broadcase_shape);
        let mut result_data : Vec<f64> = Vec::with_capacity(num_elements);
        for i in 0..num_elements{
            result_data.push(tensor1.data[i] * tensor2.data[i]);
        }
        Tensor::new(result_data, broadcase_shape)
    }

    pub fn multiply(&self, other: &Tensor) -> Tensor{
        let shape_length = self.shape.len();
        let other_length = other.shape.len();
        if shape_length < 2 || other_length < 2 {
            panic!("Shapes are small");
        }
        if self.shape[shape_length - 1] != other.shape[other_length - 2] {
            panic!("Shapes are not aligned for multiplication");
        }
        let batch_shape = self.shape[0..shape_length - 2].to_vec();
        let other_batch_shape = other.shape[0..other_length - 2].to_vec();
        let m = self.shape[shape_length - 2];
        let k1 = self.shape[shape_length - 1];
        let k2 = other.shape[other_length - 2];
        let n = other.shape[other_length - 1];
        if k1 != k2{
            panic!("Inner dimensions must match for matrix multiplication.");
        }
        let broadcast_shape: Vec<usize>;
        let self_broadcasted: Tensor;
        let other_broadcasted: Tensor;
        let (a, b): (&Tensor, &Tensor);
        if batch_shape != other_batch_shape {
            broadcast_shape = self.broadcast_shape(&batch_shape, &other_batch_shape);
            let mut self_broadcast_shape = broadcast_shape.clone();
            self_broadcast_shape.extend([m, k1]);
            let mut other_broadcast_shape = broadcast_shape.clone();
            other_broadcast_shape.extend([k2, n]);
            self_broadcasted = self.broadcast_to(&self_broadcast_shape);
            other_broadcasted = other.broadcast_to(&other_broadcast_shape);
            a = &self_broadcasted;
            b = &other_broadcasted;
        } else {
            broadcast_shape = batch_shape;
            a = self;
            b = other;
        }
        let mut result_shape = broadcast_shape.clone();
        result_shape.extend([m, n]);
        let num_elements = Tensor::compute_number_of_elements(&result_shape);
        let mut result_data : Vec<f64> = Vec::with_capacity(num_elements);
        let result_strides = Tensor::compute_strides(&result_shape);
        for i in 0..num_elements{
            let indices = Tensor::unflatten_index(i, &result_strides);
            let batch_index = indices[0..indices.len() - 2].to_vec();
            let row = indices[indices.len() - 2];
            let col = indices[indices.len() - 1];
            let mut sum_result = 0.0;
            for k in 0..k1{
                let mut a_index = batch_index.clone();
                a_index.extend([row, k]);
                let mut b_index = batch_index.clone();
                b_index.extend([k, col]);
                sum_result += a.get_value(&a_index) * b.get_value(&b_index);
            }
            result_data.push(sum_result);
        }
        Tensor::new(result_data, result_shape)
    }

    pub fn partial(&self, start_indices: Vec<usize>, end_indices: Vec<usize>) -> Tensor{
        if start_indices.len() != self.shape.len() || end_indices.len() != self.shape.len() {
            panic!("startIndices and endIndices must match the number of dimensions.");
        }
        let mut new_shape = Vec::with_capacity(self.shape.len());
        for i in 0..self.shape.len(){
            new_shape.push(end_indices[i] - start_indices[i]);
        }
        let num_elements = Tensor::compute_number_of_elements(&new_shape);
        let mut sub_data: Vec<f64> = Vec::with_capacity(num_elements);
        let new_strides = Tensor::compute_strides(&new_shape);
        for i in 0..num_elements{
            let sub_indices = Tensor::unflatten_index(i, &new_strides);
            let mut original_indices = Vec::with_capacity(self.shape.len());
            for j in 0..self.shape.len(){
                original_indices.push(start_indices[j] + sub_indices[j]);
            }
            sub_data.push(self.get_value(&original_indices));
        }
        Tensor::new(sub_data, new_shape)
    }

    pub fn get_shape(&self) -> Vec<usize>{
        self.shape.clone()
    }

    pub fn get_data(&self) -> Vec<f64>{
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::tensor::Tensor;

    #[test]
    fn test_constructor() {
        let tensor = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert_eq!(tensor.data, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_get(){
        let tensor = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        assert_eq!(1.0, tensor.get_value(&vec![0, 0]));
        assert_eq!(4.0, tensor.get_value(&vec![1, 1]));
    }

    #[test]
    fn test_set(){
        let mut tensor = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        tensor.set(&vec![0, 0], 5.0);
        assert_eq!(5.0, tensor.get_value(&vec![0, 0]));
    }

    #[test]
    fn test_reshape(){
        let tensor = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        tensor.reshape(vec![4]);
        assert_eq!(tensor.data, vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_transpose_no_axes(){
        let tensor = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let transposed = tensor.transpose(None);
        assert_eq!(transposed.data, vec![1.0, 3.0, 2.0, 4.0]);
    }

    #[test]
    fn test_transpose_with_axes(){
        let tensor = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let transposed = tensor.transpose(Some(&vec![1, 0]));
        assert_eq!(transposed.data, vec![1.0, 3.0, 2.0, 4.0]);
    }

    #[test]
    fn test_broadcast(){
        let tensor = Tensor::new(vec![1.0, 2.0], vec![1, 2]);
        let broadcasted = tensor.broadcast_to(&vec![2, 2]);
        assert_eq!(broadcasted.data, vec![1.0, 2.0, 1.0, 2.0]);
    }

    #[test]
    fn test_add_same_shape(){
        let tensor1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let tensor2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let sum = tensor1.add(&tensor2);
        assert_eq!(sum.data, vec![6.0, 8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_add_different_shape(){
        let tensor1 = Tensor::new(vec![1.0, 2.0], vec![1, 2]);
        let tensor2 = Tensor::new(vec![3.0, 4.0], vec![2, 1]);
        let sum = tensor1.add(&tensor2);
        assert_eq!(sum.data, vec![4.0, 5.0, 5.0, 6.0]);
    }

    #[test]
    fn test_subtract_same_shape(){
        let tensor1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let tensor2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let diff = tensor2.subtract(&tensor1);
        assert_eq!(diff.data, vec![4.0, 4.0, 4.0, 4.0]);
    }

    #[test]
    fn test_subtract_different_shape(){
        let tensor1 = Tensor::new(vec![5.0, 5.0], vec![2, 1]);
        let tensor2 = Tensor::new(vec![1.0, 2.0], vec![1, 2]);
        let diff = tensor1.subtract(&tensor2);
        assert_eq!(diff.data, vec![4.0, 3.0, 4.0, 3.0]);
    }

    #[test]
    fn test_hadamard_same_shape(){
        let tensor1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let tensor2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let hadamard = tensor1.hadamard_product(&tensor2);
        assert_eq!(hadamard.data, vec![5.0, 12.0, 21.0, 32.0]);
    }

    #[test]
    fn test_hadamard_different_shape(){
        let tensor1 = Tensor::new(vec![1.0, 2.0], vec![1, 2]);
        let tensor2 = Tensor::new(vec![3.0, 4.0], vec![2, 1]);
        let hadamard = tensor1.hadamard_product(&tensor2);
        assert_eq!(hadamard.data, vec![3.0, 6.0, 4.0, 8.0]);
    }

    #[test]
    fn test_multiply_2d(){
        let tensor1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]);
        let tensor2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]);
        let product = tensor1.multiply(&tensor2);
        assert_eq!(product.data, vec![19.0, 22.0, 43.0, 50.0]);
    }

    #[test]
    fn test_multiply_3d(){
        let tensor1 = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 9.0, 10.0, 11.0, 12.0], vec![2, 2, 2]);
        let tensor2 = Tensor::new(vec![5.0, 6.0, 7.0, 8.0, 13.0, 14.0, 15.0, 16.0], vec![2, 2, 2]);
        let product = tensor1.multiply(&tensor2);
        assert_eq!(product.data, vec![19.0, 22.0, 43.0, 50.0, 267.0, 286.0, 323.0, 346.0]);
    }

}
