use crate::vector::Vector;

pub struct Eigenvector {
    pub vector: Vector,
    pub eigen_value: f64
}

impl Eigenvector {
    pub(crate) fn new(eigen_value: f64, values: Vec<f64>) -> Eigenvector {
        let vector = Vector::new(values);
        Self{vector, eigen_value: eigen_value }
    }

    /// The eigenValue method which returns the eigenValue variable.
    ///
    /// returns: f32 eigenValue variable.
    ///
    pub fn eigen_value(&self) -> f64 {
        self.eigen_value
    }
}