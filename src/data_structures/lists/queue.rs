pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    head: usize,
    tail: usize,
    count: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }

        Self {
            buffer,
            capacity,
            head: 0,
            tail: 0,
            count: 0,
        }
    }

    pub fn enqueue(&mut self, elem: T) -> bool {
        if self.count == self.capacity {
            return false; // TODO: resizing of the vector can be done
        }

        self.buffer[self.tail] = Some(elem);
        self.tail = (self.tail + 1) % self.capacity;
        self.count += 1;

        return true;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        let val = self.buffer[self.head].take();
        // i think we do not need below line as take() itself takes the value
        // and leaves None in it's place;
        // self.buffer[self.head] = None;

        self.head = (self.head + 1) % self.capacity;
        self.count -= 1;

        return val;
    }
}
