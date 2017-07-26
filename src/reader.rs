use std::io::{BufRead, Read, BufReader, self, Error, ErrorKind};
use std::{cmp,mem};

use super::InflateStream;

/// A DEFLATE decoder/decompressor.
///
/// This structuree implements a `BufRead` interface and takes a stream of compressed data as input,
/// provoding the decompressed data when read from.
pub struct DeflateDecoderBuf<R> {
    /// The inner reader instance
    reader: R,
    /// The raw decompressor
    decompressor: InflateStream,
    /// How many bytes of the decompressor's output buffer still need to be output.
    pending_output_bytes: usize,
    /// Total number of bytes read from the underlying reader.
    total_in: u64,
    /// Total number of bytes written in `read` calls.
    total_out: u64,
}

impl<R: BufRead> DeflateDecoderBuf<R> {
    pub fn new(reader: R) -> DeflateDecoderBuf<R> {
        DeflateDecoderBuf {
            reader: reader,
            decompressor: InflateStream::new(),
            pending_output_bytes: 0,
            total_in: 0,
            total_out: 0,
        }
    }
}

impl<R> DeflateDecoderBuf<R> {
    /// Resets the decompressor, and replaces the current inner `BufRead` instance by `r`.
    /// without doing any extra reallocations.
    ///
    /// Note that this function doesn't ensure that all data has been output.
    pub fn reset(&mut self, r: R) -> R {
        self.decompressor.reset();
        mem::replace(&mut self.reader, r)
    }

    /// Resets the decoder, but continue to read from the same reader.
    ///
    /// Note that this function doesn't ensure that all data has been output.
    pub fn reset_data(&mut self) {
        self.decompressor.reset()
    }

    /// Returns a reference to the underlying `BufRead` instance.
    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    /// Returns a mutable reference to the underlying `BufRead` instance.
    ///
    /// Note that mutation of the reader may cause surprising results if the decoder is going to
    /// keep being used.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Drops the decoder and return the inner `BufRead` instance.
    ///
    /// Note that this function doesn't ensure that all data has been output.
    pub fn into_inner(self) -> R {
        self.reader
    }

    /// Returns the total bytes read from the underlying `BufRead` instance.
    pub fn total_in(&self) -> u64 {
        self.total_in
    }

    /// Returns the total number of bytes output from this decoder.
    pub fn total_out(&self) -> u64 {
        self.total_out
    }
}

impl<R: BufRead> Read for DeflateDecoderBuf<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut bytes_out = 0;
        // If there is still data left to ouput from the last call to `update()`, that needs to be
        // output first
        if self.pending_output_bytes != 0 {
            // Get the part of the buffer that has not been output yet.
            // The decompressor sets `pos` to 0 when it reaches the end of it's internal buffer,
            // so we have to check for that.
            let start = if self.decompressor.pos != 0 {
                self.decompressor.pos as usize - self.pending_output_bytes
            } else {
                self.decompressor.buffer.len() - self.pending_output_bytes
            };

            // Copy as much decompressed as possible to buf.
            let bytes_to_copy = cmp::min(buf.len(), self.pending_output_bytes);
            let pending_data =
                &self.decompressor.buffer[start..
                                          start + bytes_to_copy];
            buf[..bytes_to_copy].copy_from_slice(pending_data);
            bytes_out += bytes_to_copy;
            // This won't underflow since `bytes_to_copy` will be at most
            // the same value as `pending_output_bytes`.
            self.pending_output_bytes -= bytes_to_copy;
            if self.pending_output_bytes != 0 {
                self.total_out += bytes_out as u64;
                // If there is still decompressed data left that didn't
                // fit in `buf`, return what we read.
                return Ok(bytes_out);
            }
        }

        // There is space in `buf` for more data, so try to read more.
        let (input_bytes_read, remaining_bytes) = {
            self.pending_output_bytes = 0;
            let input = try!(self.reader.fill_buf());
            if input.len() == 0 {
                self.total_out += bytes_out as u64;
                //If there is nothing more to read, return.
                return Ok(bytes_out);
            }
            let (input_bytes_read, data) =
                match self.decompressor.update(&input) {
                    Ok(res) => res,
                    Err(m) => return Err(Error::new(ErrorKind::Other, m))
                };

            // Space left in `buf`
            let space_left = buf.len() - bytes_out;
            let bytes_to_copy = cmp::min(space_left, data.len());
            buf[bytes_out..bytes_out + bytes_to_copy].copy_from_slice(&data[..bytes_to_copy]);

            bytes_out += bytes_to_copy;

            // Can't underflow as bytes_to_copy is bounded by data.len().
            (input_bytes_read, data.len() - bytes_to_copy)

        };

        self.pending_output_bytes += remaining_bytes;
        self.total_in += input_bytes_read as u64;
        self.total_out += bytes_out as u64;
        self.reader.consume(input_bytes_read);

        Ok(bytes_out)
    }
}



/// A DEFLATE decoder/decompressor.
///
/// This structuree implements a `Read` interface and takes a stream of compressed data as input,
/// provoding the decompressed data when read from.
pub struct DeflateDecoder<R> {
    /// Inner DeflateDecoderBuf, with R wrapped in a `BufReader`.
    inner: DeflateDecoderBuf<BufReader<R>>
}

impl<R: Read> DeflateDecoder<R> {
    pub fn new(reader: R) -> DeflateDecoder<R> {
        DeflateDecoder {
            inner: DeflateDecoderBuf::new(BufReader::new(reader))
        }
    }

    /// Resets the decompressor, and replaces the current inner `BufRead` instance by `r`.
    /// without doing any extra reallocations.
    ///
    /// Note that this function doesn't ensure that all data has been output.
    pub fn reset(&mut self, r: R) -> R {
        self.inner.reset(BufReader::new(r)).into_inner()
    }

    /// Returns a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        self.inner.get_ref().get_ref()
    }

    /// Returns a mutable reference to the underlying reader.
    ///
    /// Note that mutation of the reader may cause surprising results if the decoder is going to
    /// keep being used.
    pub fn get_mut(&mut self) -> &mut R {
        self.inner.get_mut().get_mut()
    }

    /// Returns the total number of bytes output from this decoder.
    pub fn into_inner(self) -> R {
        self.inner.into_inner().into_inner()
    }
}

impl<R> DeflateDecoder<R> {
    /// Resets the decoder, but continue to read from the same reader.
    ///
    /// Note that this function doesn't ensure that all data has been output.
    pub fn reset_data(&mut self) {
        self.inner.reset_data()
    }

    /// Returns the total bytes read from the underlying reader.
    pub fn total_in(&self) -> u64 {
        self.inner.total_in
    }

    /// Returns the total number of bytes output from this decoder.
    pub fn total_out(&self) -> u64 {
        self.inner.total_out
    }
}

impl<R: BufRead> Read for DeflateDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

#[cfg(test)]
mod test {
    use super::{DeflateDecoder};

    #[test]
    fn deflate_reader() {
        let encoded = vec![243, 72, 205, 201, 201, 215, 81, 40, 207, 47, 202, 73, 1, 0];
        let mut decoder = DeflateDecoder::new(encoded.as_slice());
        let mut output = Vec::new();
        decoder.read_to_end(&mut output).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Hello, world");
    }
}
