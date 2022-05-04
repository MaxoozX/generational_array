use std::fmt;
use std::collections::VecDeque;

#[derive(Copy, Clone)] // FIXME: Do I really want it to be clonable and copiable
pub struct GenerationalIndex {
    index: usize,
    generation: usize,
}

impl fmt::Display for GenerationalIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(index: {}, generation: {})", self.index, self.generation)
    }
}

#[derive(Debug)]
pub struct GenerationalArray<ValueType> {

    values: Vec<Option<ValueType>>,
    generations: Vec<usize>,
    free_indices: VecDeque<usize>,
    size: usize,
    used_size: usize,

}

#[derive(Debug)]
pub enum GenerationalArrayResult<'a, ValueType> {
    None,
    OutDated,
    OutOfBounds,
    Some(&'a ValueType),
}

#[derive(Debug)]
pub enum GenerationalArrayResultMut<'a, ValueType> {
    None,
    OutDated,
    OutOfBounds,
    Some(&'a mut ValueType),
}

impl<ValueType> GenerationalArray<ValueType> {

    pub fn new() -> GenerationalArray<ValueType> {
        GenerationalArray::<ValueType> {
            values: Vec::<Option<ValueType>>::new(),
            generations: Vec::<usize>::new(),
            free_indices: VecDeque::<usize>::new(),
            size: 0,
            used_size: 0,
        }
    }

    pub fn insert(&mut self, value: ValueType) -> GenerationalIndex {

        // Find the index to put our new value
        let next_index: usize = match self.free_indices.pop_front() {
            None => {
                // No more empty space, grow the index by one
                self.values.push(None);
                self.generations.push(0);
                self.size += 1;
                self.size - 1
            }
            Some(idx) => {
                idx
            }
        };

        // Add the value
        self.values[next_index] = Some(value);
        self.used_size += 1;
        // generations[next_index] += 1;

        // return the index
        GenerationalIndex { index: next_index, generation: self.generations[next_index] }

    }

    pub fn remove(&mut self, index: GenerationalIndex) -> Result<_, &'static str> {

        // Validate the generation
        if index.generation != self.generations[index.index] {
            return Err("The generation is outdated");
        }

        // Validate index
        if index.index >=self.size{
            return Err("The index is out of range");
        }

        // Remove the element
        self.values[index.index] = None;
        self.generations[index.index] += 1;

        self.free_indices.push_back(index.index);
        self.used_size -= 1;

        Ok(())

    }

    pub fn get(&self, index: &GenerationalIndex) -> GenerationalArrayResult<ValueType> {

        // Validate the generation
        if index.generation != self.generations[index.index] {
            return GenerationalArrayResult::OutDated;
        }

        // Validate index
        if index.index >=self.size{
            return GenerationalArrayResult::OutOfBounds;
        }

        match &self.values[index.index] {
            None => GenerationalArrayResult::None,
            Some(val) => GenerationalArrayResult::Some(val),
        }

    }

    pub fn get_mut(&mut self, index: &GenerationalIndex) -> GenerationalArrayResultMut<ValueType> {

        // Validate the generation
        if index.generation != self.generations[index.index] {
            return GenerationalArrayResultMut::OutDated;
        }

        // Validate index
        if index.index >= self.size{
            return GenerationalArrayResultMut::OutOfBounds;
        }

        if self.values[index.index].is_none() {
            return GenerationalArrayResultMut::None;
        }

        GenerationalArrayResultMut::Some(self.values[index.index].as_mut().unwrap())

    }

    pub fn is_empty(&self) -> bool {
        self.used_size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn used_size(&self) -> usize {
        self.used_size
    }

}