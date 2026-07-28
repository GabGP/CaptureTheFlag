use crate::{
    debugger::{LogDirection, log_message},
    protocol::protocol,
};
use std::io::{self, Read, Write};

// ============================================================================
// TCP NETWORK UTILITIES
// ============================================================================

/// Function to bundle and send a message securely over a TCP connection
pub fn send_frame<W: Write>(
    writer: &mut W,
    msg: &protocol::Message,
    side: &str,
    address: &str,
) -> io::Result<()> {
    let payload = msg.serialize();
    let len = payload.len() as u16;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(&payload)?;
    writer.flush()?;
    log_message(address, side, LogDirection::Sent, msg);
    Ok(())
}

/// Function to receive and decode an incoming message from a TCP connection
pub fn read_frame<R: Read>(
    reader: &mut R,
    side: &str,
    address: &str,
) -> io::Result<protocol::Message> {
    let mut len_bytes = [0u8; 2];
    reader.read_exact(&mut len_bytes)?;
    let len = u16::from_be_bytes(len_bytes) as usize;

    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    let message = protocol::Message::deserialize(&buf)?;
    log_message(address, side, LogDirection::Received, &message);
    Ok(message)
}
