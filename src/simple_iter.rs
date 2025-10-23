use crate::io;

pub struct LineColIterator<I> {
    iter: I,
}

impl<I> LineColIterator<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    pub fn new(iter: I) -> LineColIterator<I> {
        LineColIterator {
            iter,
        }
    }

    pub fn line(&self) -> usize {
        0
    }

    pub fn col(&self) -> usize {
        0
    }

    pub fn byte_offset(&self) -> usize {
        1
    }
}

impl<I> Iterator for LineColIterator<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<io::Result<u8>> {
        self.iter.next()
    }
}
