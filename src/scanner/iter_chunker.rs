#[derive(Debug)]
pub struct IterChunker<T> {
    iter: T,
    size: usize,
}

impl<I> IterChunker<I> {
    pub fn new(iter: I, size: usize) -> IterChunker<I> {
        IterChunker { iter, size }
    }
}

impl<T> Iterator for IterChunker<T>
where
    T: Iterator,
{
    type Item = Vec<<T>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let IterChunker { iter, ref size } = self;
        let chunk = iter.take(*size).collect::<Self::Item>();
        match chunk {
            c if c.is_empty() => None,
            _ => Some(chunk),
        }
    }
}

pub trait ChunkExt: Iterator {
    fn chunk(self, size: usize) -> IterChunker<Self>
    where
        Self: Sized,
    {
        IterChunker::new(self, size)
    }
}

impl<I: Iterator> ChunkExt for I {}
