use std::io;

// ============================================================================
// UTILITIES
// ============================================================================

/// Function to convert a floating-point number into a 32-bit integer for network transport
pub fn float_to_i32(val: f32) -> i32 {
    (val * 100.0).round() as i32
}

/// Function to convert a 32-bit integer back into a floating-point number
pub fn i32_to_float(val: i32) -> f32 {
    val as f32 / 100.0
}

// ============================================================================
// BYTE WRITER
// ============================================================================

pub struct ByteWriter {
    buf: Vec<u8>,
}

impl ByteWriter {
    /// Function to create a new, empty ByteWriter buffer
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Function to append a single 8-bit unsigned integer to the buffer
    pub fn write_u8(&mut self, val: u8) {
        self.buf.push(val);
    }

    /// Function to append a 16-bit unsigned integer to the buffer
    pub fn write_u16(&mut self, val: u16) {
        self.buf.extend_from_slice(&val.to_be_bytes());
    }

    /// Function to append a 32-bit unsigned integer to the buffer
    pub fn write_u32(&mut self, val: u32) {
        self.buf.extend_from_slice(&val.to_be_bytes());
    }

    /// Function to append a 32-bit signed integer to the buffer
    pub fn write_i32(&mut self, val: i32) {
        self.buf.extend_from_slice(&val.to_be_bytes());
    }

    /// Function to append a boolean value to the buffer
    pub fn write_bool(&mut self, val: bool) {
        self.buf.push(if val { 1 } else { 0 });
    }

    /// Function to append a text string to the buffer
    pub fn write_str(&mut self, val: &str) {
        let bytes = val.as_bytes();
        let len = bytes.len().min(255) as u8;
        self.buf.push(len);
        self.buf.extend_from_slice(&bytes[..len as usize]);
    }

    /// Function to consume the writer and return the final byte array
    pub fn into_bytes(self) -> Vec<u8> {
        self.buf
    }
}

// ============================================================================
// BYTE READER
// ============================================================================

pub struct ByteReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ByteReader<'a> {
    /// Function to create a new ByteReader for parsing an existing byte array
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Function to read a single 8-bit unsigned integer from the current buffer position
    pub fn read_u8(&mut self) -> Result<u8, io::Error> {
        if self.pos + 1 > self.buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer overflow reading u8",
            ));
        }
        let val = self.buf[self.pos];
        self.pos += 1;
        Ok(val)
    }

    /// Function to read a 16-bit unsigned integer from the current buffer position
    pub fn read_u16(&mut self) -> Result<u16, io::Error> {
        if self.pos + 2 > self.buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer overflow reading u16",
            ));
        }
        let arr = [self.buf[self.pos], self.buf[self.pos + 1]];
        self.pos += 2;
        Ok(u16::from_be_bytes(arr))
    }

    /// Function to read a 32-bit unsigned integer from the current buffer position
    pub fn read_u32(&mut self) -> Result<u32, io::Error> {
        if self.pos + 4 > self.buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer overflow reading u32",
            ));
        }
        let arr = [
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
        ];
        self.pos += 4;
        Ok(u32::from_be_bytes(arr))
    }

    /// Function to read a 32-bit signed integer from the current buffer position
    pub fn read_i32(&mut self) -> Result<i32, io::Error> {
        if self.pos + 4 > self.buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer overflow reading i32",
            ));
        }
        let arr = [
            self.buf[self.pos],
            self.buf[self.pos + 1],
            self.buf[self.pos + 2],
            self.buf[self.pos + 3],
        ];
        self.pos += 4;
        Ok(i32::from_be_bytes(arr))
    }

    /// Function to read a boolean value from the current buffer position
    pub fn read_bool(&mut self) -> Result<bool, io::Error> {
        let val = self.read_u8()?;
        Ok(val != 0)
    }

    /// Function to read a text string from the current buffer position
    pub fn read_str(&mut self) -> Result<String, io::Error> {
        let len = self.read_u8()? as usize;
        if self.pos + len > self.buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Buffer overflow reading str",
            ));
        }
        let bytes = &self.buf[self.pos..self.pos + len];
        self.pos += len;
        String::from_utf8(bytes.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
