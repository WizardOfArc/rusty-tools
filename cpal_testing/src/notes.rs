use std::collections::HashMap;

#[derive(Debug)]
pub struct Melody {
    notes: HashMap<usize, f32>,
}

impl Melody {
    pub fn mapping(&self, length_in_subdivisions: usize) -> Vec<f32> {
        let mut frequencies: Vec<f32> = vec![-1.0; length_in_subdivisions];
        let mut cur_freq = 0.0;
        let mut f_idx = 0;
        while f_idx < length_in_subdivisions {
            match self.notes.get(&f_idx) {
                None => frequencies[f_idx] = cur_freq.clone(),
                Some(new_freq) => {
                    frequencies[f_idx] = new_freq.clone();
                    cur_freq = new_freq.clone();
                }
            }
            f_idx += 1;
        }
        frequencies
    }

    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, sub_idx: usize, freq: f32) {
        self.notes.insert(sub_idx, freq);
    }

    pub fn from_list_of_tuples(list: Vec<(usize, f32)>) -> Self {
        let mut new_self = Self::new();
        for (sub_idx, freq) in list {
            new_self.add_entry(sub_idx, freq);
        }
        new_self
    }
}
