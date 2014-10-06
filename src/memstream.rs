use std::io;
use std::slice;
use std::cmp::min;
use std::slice::Slice;
use std::io::IoResult;
use std::collections::RingBuf;

pub struct MemStream {
    buf: RingBuf<u8>,
    pos: uint    
}

#[deriving(PartialOrd)]
impl MemStream {
    /// Creates a new `MemStream` which can be read and 
    /// written to 
    fn new(buf: RingBuf<u8>) -> MemStream {
        MemStream {
            buf: buf,
            pos: 0 
        }
    }
    /// Tests whether this stream has read all bytes in its ring buffer
    /// If `true`, then this will no longer return bytes from `read`
    fn eof(&self) -> bool { self.pos >= self.buf.len() }
}

impl Reader for MemStream {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<uint> {
        if self.eof() { return Err(io::standard_error(io::EndOfFile)) }
        let write_len = min(buf.len(), self.buf.len() - self.pos);
        {   
            let input = self.buf.slice(self.pos, self.pos + write_len);
            let output = buf.slice_mut(0, write_len);
            assert_eq!(input.len(), output.len());
            slice::bytes::copy_memory(output, input);
        }
        self.pos += write_len;
        assert!(self.pos <= self.buf.len());

        return Ok(write_len);
    }
}

impl Writer for MemStream {
    fn write(&mut self, buf: &[u8]) -> IoResult<()> {
        self.buf.push_all(buf);
        Ok(())
    }
}

// #[cfg(test)]
// mod test {
//     use std::io;
//     use memstream::MemStream;

//     #[test]
//     fn test_mem_stream() {

//     }
// }
