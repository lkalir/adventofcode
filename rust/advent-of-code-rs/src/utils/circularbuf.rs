use std::mem::MaybeUninit;

pub struct CircularBuf<T, const K: usize> {
    items: [MaybeUninit<T>; K],
    head: usize,
    tail: usize,
}

impl<T: std::fmt::Debug, const K: usize> std::fmt::Debug for CircularBuf<T, K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for item in self.items.as_ref() {
            writeln!(f, "\t{:?},", unsafe { item.assume_init_ref() })?;
        }

        write!(f, "]")
    }
}

impl<T, const K: usize> CircularBuf<T, K> {
    pub fn iter(&self) -> CircularBufIter<T, K> {
        CircularBufIter {
            buf: self,
            idx: self.tail,
        }
    }
}

impl<T: Copy, const K: usize> CircularBuf<T, K> {
    pub fn new() -> Self {
        Self {
            items: [MaybeUninit::uninit(); K],
            head: 0,
            tail: 0,
        }
    }

    pub fn insert(&mut self, val: T) -> T {
        unsafe {
            let last = self.items[self.head % K].assume_init();
            *self.items[self.head % K].assume_init_mut() = val;
            self.head += 1;

            if self.head >= K {
                self.tail += 1;
            }

            last
        }
    }
}

#[derive(Debug)]
pub struct CircularBufIter<'a, T, const K: usize> {
    buf: &'a CircularBuf<T, K>,
    idx: usize,
}

impl<'a, T: Copy, const K: usize> Iterator for CircularBufIter<'a, T, K> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.buf.head {
            None
        } else {
            let ret = unsafe { self.buf.items[self.idx % K].assume_init() };
            self.idx += 1;
            Some(ret)
        }
    }
}
