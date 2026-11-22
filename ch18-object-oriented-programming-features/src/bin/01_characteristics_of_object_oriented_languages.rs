mod avg {
    #[derive(Debug)]
    pub struct AveragedCollection {
        list: Vec<i32>,
        average: f64,
    }

    impl AveragedCollection {
        pub fn new(list: Vec<i32>) -> Self {
            let mut avg_col = Self { list, average: 0.0 };
            avg_col.update_average();
            avg_col
        }

        pub fn add(&mut self, value: i32) {
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }
                None => None,
            }
        }

        pub fn average(&self) -> f64 {
            self.average
        }

        fn update_average(&mut self) {
            let total: i32 = self.list.iter().sum();
            self.average = total as f64 / self.list.len() as f64;
        }
    }
}

fn main() {
    let mut avg_col = avg::AveragedCollection::new(vec![1, 2, 3]);
    println!("{avg_col:?}");
    avg_col.add(4);
    println!("{avg_col:?}");
    avg_col.remove();
    println!("{avg_col:?}");
    println!("avg: {}", avg_col.average());
}
