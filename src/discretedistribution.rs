use std::collections::{BTreeMap, HashMap};

pub struct DiscreteDistribution {
    map: BTreeMap<String, i32>,
    sum: i32
}

impl DiscreteDistribution {
    pub fn new() -> DiscreteDistribution {
        Self{map: BTreeMap::new(), sum: 0}
    }

    /// The addItem method takes a String item as an input and if this map contains a mapping for the item it puts the item
    /// with given value + 1, else it puts item with value of 1.
    ///
    /// # Arguments
    ///
    /// * `item`: String input
    ///
    /// returns: ()
    ///
    pub fn add_item(&mut self, item: &String) {
        if self.map.contains_key(item) {
            let value = self.map.get(item).unwrap();
            self.map.insert(item.clone(), value + 1);
        } else {
            self.map.insert(item.clone(), 1);
        }
        self.sum += 1;
    }

    /// The removeItem method takes a String item as an input and if this map contains a mapping for the item it puts the item
    /// with given value - 1, and if its value is 0, it removes the item.
    ///
    /// # Arguments
    ///
    /// * `item`: String input.
    ///
    /// returns: ()
    ///
    pub fn remove_item(&mut self, item: &String) {
        if self.map.contains_key(item) {
            let value = self.map.get(item).unwrap();
            if *value == 1{
                self.map.remove(item);
            } else {
                self.map.insert(item.clone(), value - 1);
            }
            self.sum -= 1;
        }
    }

    /// The addDistribution method takes a DiscreteDistribution as an input and loops through the entries in this distribution
    /// and if this map contains a mapping for the entry it puts the entry with its value + entry, else it puts entry with its value.
    /// It also accumulates the values of entries and assigns to the sum variable.
    ///
    /// # Arguments
    ///
    /// * `distribution`: DiscreteDistribution type input.
    ///
    /// returns: ()
    ///
    pub fn add_distribution(&mut self, distribution: &DiscreteDistribution) {
        for entry in distribution.map.iter() {
            if self.map.contains_key(entry.0) {
                let value = self.map.get(entry.0).unwrap();
                self.map.insert(entry.0.to_string(), value + entry.1);
            } else {
                self.map.insert(entry.0.to_string(), *entry.1);
            }
            self.sum += *entry.1;
        }
    }

    /// The removeDistribution method takes a {@link DiscreteDistribution} as an input and loops through the entries in this distribution
    /// and if this map contains a mapping for the entry it puts the entry with its key - value, else it removes the entry.
    /// It also decrements the value of entry from sum and assigns to the sum variable.
    ///
    /// # Arguments
    ///
    /// * `distribution`: DiscreteDistribution type input.
    ///
    /// returns: ()
    ///
    pub fn remove_distribution(&mut self, distribution: &DiscreteDistribution) {
        for entry in distribution.map.iter() {
            if self.map.contains_key(entry.0) {
                let value = self.map.get(entry.0).unwrap();
                if *value - *entry.1 != 0 {
                    self.map.insert(entry.0.to_string(), value - entry.1);
                } else {
                    self.map.remove(entry.0);
                }
            }
            self.sum -= *entry.1;
        }
    }

    /// The getter for sum variable.
    ///
    /// returns: sum
    ///
    pub fn get_sum(&self) -> i32 {
        self.sum
    }

    /// The getIndex method takes an item as an input and returns the index of given item.
    ///
    /// # Arguments
    ///
    /// * `item`: to search for index.
    ///
    /// returns: usize index of given item.
    ///
    pub fn get_index(&self, item: &String) -> Option<usize> {
        self.map
            .iter()
            .position(|(key, _)| key == item)
    }

    /// The containsItem method takes an item as an input and returns true if this map contains a mapping for the
    /// given item.
    ///
    /// # Arguments
    ///
    /// * `item`: item to check.
    ///
    /// returns: bool true if this map contains a mapping for the given item.
    ///
    pub fn contains_item(&self, item: &String) -> bool {
        self.map.contains_key(item)
    }

    /// The getItem method takes an index as an input and returns the item at given index.
    ///
    /// # Arguments
    ///
    /// * `index`: is used for searching the item.
    ///
    /// returns: Option<String> the item at given index.
    ///
    pub fn get_item(&self, index: usize) -> Option<String> {
        let mut i = 0;
        for (key, _value) in &self.map {
            if i == index {
                return Some(key.clone());
            }
            i = i + 1;
        }
        None
    }

    /// The getValue method takes an index as an input and returns the value at given index.
    ///
    /// # Arguments
    ///
    /// * `index`: is used for searching the value.
    ///
    /// returns: Option<i32> the value at given index.
    ///
    pub fn get_value(&self, index: usize) -> Option<i32> {
        let mut i = 0;
        for (_key, value) in &self.map {
            if i == index {
                return Some(*value);
            }
            i = i + 1;
        }
        None
    }

    /// The getCount method takes an item as an input returns the value to which the specified item is mapped, or {@code null}
    /// if this map contains no mapping for the key.
    ///
    /// # Arguments
    ///
    /// * `item`: is used to search for value.
    ///
    /// returns: Option<&i32> the value to which the specified item is mapped
    ///
    pub fn get_count(&self, item: &String) -> i32 {
        if self.map.contains_key(item) {
            return *self.map.get(item).unwrap()
        }
        0
    }

    /// The getMaxItem method loops through the entries and gets the entry with maximum value.
    ///
    /// returns: Option<&String> the entry with maximum value.
    ///
    pub fn get_max_item(&self) -> Option<String> {
        let mut max = -1;
        let mut max_item = None;
        for (key, value) in &self.map {
            if *value > max {
                max = *value;
                max_item = Some(key.clone())
            }
        }
        max_item
    }

    /// Another getMaxItem method which takes a vector of Strings. It loops through the items in this
    /// vector and gets the item with maximum value.
    ///
    /// # Arguments
    ///
    /// * `include_these_only`: vector of strings
    ///
    /// returns: Option<&String> the item with maximum value.
    ///
    pub fn get_max_item_include_only(&self, include_these_only: Vec<String>) -> Option<String> {
        let mut max = -1;
        let mut max_item = None;
        for item in include_these_only {
            let mut frequency = 0;
            if self.contains_item(&item) {
                frequency = *self.map.get(&item).unwrap();
            }
            if frequency > max {
                max = frequency;
                max_item = Some(item);
            }
        }
        max_item
    }

    /// The getProbability method takes an item as an input returns the value to which the specified item is mapped over sum,
    /// or 0.0 if this map contains no mapping for the key.
    ///
    /// # Arguments
    ///
    /// * `item`: is used to search for probability.
    ///
    /// returns: f64 the probability to which the specified item is mapped.
    ///
    pub fn get_probability(&self, item: &String) -> f64 {
        if self.map.contains_key(item) {
            *self.map.get(item).unwrap() as f64 / self.sum as f64
        } else {
            0.0
        }
    }

    /// Returns the distribution as a probability distribution
    ///
    /// returns: HashMap<String, f64> Probability distribution
    ///
    pub fn get_probability_distribution(&self) -> HashMap<String, f64> {
        let mut result = HashMap::new();
        for key in self.map.keys() {
            result.insert(key.clone(), self.get_probability(key));
        }
        result
    }

    /// The getProbabilityLaplaceSmoothing method takes an item as an input returns the smoothed value to which the specified
    /// item is mapped over sum, or 1.0 over sum if this map contains no mapping for the key.
    ///
    /// # Arguments
    ///
    /// * `item`: is used to search for probability.
    ///
    /// returns: f64 the smoothed probability to which the specified item is mapped.
    ///
    pub fn get_probability_laplace_smoothing(&self, item: &String) -> f64 {
        if self.map.contains_key(item) {
            (*self.map.get(item).unwrap() as f64 + 1.0) / (self.sum as f64 + self.map.iter().len() as f64 + 1.0)
        } else {
            1.0 / (self.sum as f64 + self.map.iter().len() as f64 + 1.0)
        }
    }

    /// The entropy method loops through the values and calculates the entropy of these values.
    ///
    /// returns: f64 entropy value.
    ///
    pub fn entropy(&self) -> f64{
        let mut total = 0.0;
        for value in self.map.values() {
            let probability = *value as f64 / self.sum as f64;
            total += -probability * probability.log2()
        }
        total
    }

    pub fn size(&self) -> usize {
        self.map.len()
    }

}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use crate::discretedistribution::DiscreteDistribution;

    #[test]
    fn test_add_item_1() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.get_count(&String::from("item1")), 3);
        assert_eq!(small_distribution.get_count(&String::from("item2")), 2);
        assert_eq!(small_distribution.get_count(&String::from("item3")), 1);
    }

    #[test]
    fn test_add_item_2() {
        let mut discrete_distribution = DiscreteDistribution::new();
        for _i in 0..1000 {
            let value = rand::random_range(0..1000);
            discrete_distribution.add_item(&String::from(value.to_string()));
        }
        assert_eq!(discrete_distribution.get_sum(), 1000);
    }

    #[test]
    fn test_add_item_3() {
        let mut discrete_distribution = DiscreteDistribution::new();
        for _i in 0..1000000 {
            let value = rand::random_range(0..1000000);
            discrete_distribution.add_item(&String::from(value.to_string()));
        }
        assert_relative_eq!(discrete_distribution.size() as f64 / 1000000.0, 0.632, epsilon = 0.001);
    }

    #[test]
    fn test_remove_item() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.remove_item(&String::from("item1"));
        small_distribution.remove_item(&String::from("item2"));
        small_distribution.remove_item(&String::from("item3"));
        assert_eq!(small_distribution.get_count(&String::from("item1")), 2);
        assert_eq!(small_distribution.get_count(&String::from("item2")), 1);
    }

    #[test]
    fn test_add_distribution_1() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        let mut discrete_distribution = DiscreteDistribution::new();
        discrete_distribution.add_item(&String::from("item4"));
        discrete_distribution.add_item(&String::from("item5"));
        discrete_distribution.add_item(&String::from("item5"));
        discrete_distribution.add_item(&String::from("item2"));
        small_distribution.add_distribution(&discrete_distribution);
        assert_eq!(small_distribution.get_count(&String::from("item1")), 3);
        assert_eq!(small_distribution.get_count(&String::from("item2")), 3);
        assert_eq!(small_distribution.get_count(&String::from("item3")), 1);
        assert_eq!(small_distribution.get_count(&String::from("item4")), 1);
        assert_eq!(small_distribution.get_count(&String::from("item5")), 2);
    }

    #[test]
    fn test_add_distribution_2() {
        let mut discrete_distribution_1 = DiscreteDistribution::new();
        for i in 0..1000 {
            discrete_distribution_1.add_item(&String::from(i.to_string()));
        }
        let mut discrete_distribution_2 = DiscreteDistribution::new();
        for i in 500..1000 {
            discrete_distribution_2.add_item(&String::from((1000 + i).to_string()));
        }
        discrete_distribution_1.add_distribution(&discrete_distribution_2);
        assert_eq!(discrete_distribution_1.size(), 1500);
    }

    #[test]
    fn test_remove_distribution() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        let mut discrete_distribution = DiscreteDistribution::new();
        discrete_distribution.add_item(&String::from("item1"));
        discrete_distribution.add_item(&String::from("item1"));
        discrete_distribution.add_item(&String::from("item2"));
        small_distribution.remove_distribution(&discrete_distribution);
        assert_eq!(small_distribution.get_count(&String::from("item1")), 1);
        assert_eq!(small_distribution.get_count(&String::from("item2")), 1);
        assert_eq!(small_distribution.get_count(&String::from("item3")), 1);
    }

    #[test]
    fn test_get_index() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.get_index(&String::from("item1")).unwrap(), 0);
        assert_eq!(small_distribution.get_index(&String::from("item2")).unwrap(), 1);
        assert_eq!(small_distribution.get_index(&String::from("item3")).unwrap(), 2);
    }

    #[test]
    fn test_contains_item() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.contains_item(&String::from("item1")), true);
        assert_eq!(small_distribution.contains_item(&String::from("item4")), false);
    }

    #[test]
    fn test_get_item() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.get_item(0).unwrap(), "item1");
        assert_eq!(small_distribution.get_item(1).unwrap(), "item2");
        assert_eq!(small_distribution.get_item(2).unwrap(), "item3");
    }

    #[test]
    fn test_get_value() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.get_value(0).unwrap(), 3);
        assert_eq!(small_distribution.get_value(1).unwrap(), 2);
        assert_eq!(small_distribution.get_value(2).unwrap(), 1);
    }

    #[test]
    fn test_max_item_1() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.get_max_item().unwrap(), "item1");
    }

    #[test]
    fn test_max_item_2() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        let mut include = Vec::new();
        include.push(String::from("item2"));
        include.push(String::from("item3"));
        assert_eq!(small_distribution.get_max_item_include_only(include).unwrap(), "item2");
    }

    #[test]
    fn test_get_probability() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.get_probability(&String::from("item1")), 0.5);
        assert_eq!(small_distribution.get_probability(&String::from("item2")), 1.0 / 3.0);
        assert_eq!(small_distribution.get_probability(&String::from("item3")), 1.0 / 6.0);
    }

    #[test]
    fn test_get_probability_laplace_smoothing() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_eq!(small_distribution.get_probability_laplace_smoothing(&String::from("item1")), 0.4);
        assert_eq!(small_distribution.get_probability_laplace_smoothing(&String::from("item2")), 0.3);
        assert_eq!(small_distribution.get_probability_laplace_smoothing(&String::from("item3")), 0.2);
        assert_eq!(small_distribution.get_probability_laplace_smoothing(&String::from("item4")), 0.1);
    }

    #[test]
    fn test_entropy() {
        let mut small_distribution = DiscreteDistribution::new();
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item3"));
        small_distribution.add_item(&String::from("item1"));
        small_distribution.add_item(&String::from("item2"));
        small_distribution.add_item(&String::from("item1"));
        assert_relative_eq!(small_distribution.entropy(), 1.4591, epsilon = 0.0001);
    }

}