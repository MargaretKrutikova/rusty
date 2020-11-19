pub struct CircularBuffer<T> {
    data: Vec<T>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    fn is_full(&self) -> bool {
        self.data.len() == self.data.capacity()
    }
    fn dequeue(&mut self) -> T {
        self.data.remove(0)
    }
    fn push(&mut self, element: T) {
        self.data.push(element);
    }
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        match self.is_full() {
            true => Err(Error::FullBuffer),
            false => Ok(self.push(element)),
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.data.is_empty() {
            true => Err(Error::EmptyBuffer),
            false => Ok(self.dequeue()),
        }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn overwrite(&mut self, element: T) {
        if self.is_full() {
            self.dequeue();
        }
        self.push(element);
    }
}
