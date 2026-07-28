use crate::{
    debugger::{LogDirection, log_message},
    protocol::protocol,
};
use std::io::{self, Read, Write};
use std::thread;
use std::time::Duration;

// ============================================================================
// TCP NETWORK UTILITIES
// ============================================================================

/// Function to write all bytes to a non-blocking writer, retrying on WouldBlock or Interrupted errors
fn write_all_with_retry<W: Write>(writer: &mut W, buf: &[u8]) -> io::Result<()> {
    let mut written = 0usize;
    while written < buf.len() {
        match writer.write(&buf[written..]) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write any bytes",
                ));
            }
            Ok(n) => written += n,
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(err) if err.kind() == io::ErrorKind::Interrupted => {}
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

/// Function to flush a non-blocking writer, retrying on WouldBlock or Interrupted errors
fn flush_with_retry<W: Write>(writer: &mut W) -> io::Result<()> {
    loop {
        match writer.flush() {
            Ok(()) => return Ok(()),
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(err) if err.kind() == io::ErrorKind::Interrupted => {}
            Err(err) => return Err(err),
        }
    }
}

/// Function to bundle and send a message securely over a TCP connection
pub fn send_frame<W: Write>(
    writer: &mut W,
    msg: &protocol::Message,
    side: &str,
    address: &str,
) -> io::Result<()> {
    let payload = msg.serialize();
    let len = payload.len() as u16;
    write_all_with_retry(writer, &len.to_be_bytes())?;
    write_all_with_retry(writer, &payload)?;
    flush_with_retry(writer)?;
    log_message(address, side, LogDirection::Sent, msg);
    Ok(())
}

/// Function to read exact bytes from a non-blocking reader, waiting and retrying on WouldBlock errors
fn read_exact_or_wait<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<()> {
    let mut read = 0usize;
    while read < buf.len() {
        match reader.read(&mut buf[read..]) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "connection closed while reading frame",
                ));
            }
            Ok(n) => read += n,
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(5));
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

/// Function to receive and decode an incoming message from a TCP connection
pub fn read_frame<R: Read>(
    reader: &mut R,
    side: &str,
    address: &str,
) -> io::Result<protocol::Message> {
    let mut len_bytes = [0u8; 2];
    read_exact_or_wait(reader, &mut len_bytes)?;
    let len = u16::from_be_bytes(len_bytes) as usize;

    let mut buf = vec![0u8; len];
    read_exact_or_wait(reader, &mut buf)?;
    let message = protocol::Message::deserialize(&buf)?;
    log_message(address, side, LogDirection::Received, &message);
    Ok(message)
}
