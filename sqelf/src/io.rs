use std::io;

pub trait MemRead {
    type Reader: io::Read;

    fn bytes(&self) -> Option<&[u8]>;
    fn into_reader(self) -> io::Result<Self::Reader>;
}