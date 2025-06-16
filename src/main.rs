use std::{
    io::{Read, Write},
    net::TcpStream,
};
use bytes::{BufMut, Bytes, BytesMut};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut io = TcpStream::connect("127.0.0.1:3000")?;
    let mut buf = BytesMut::new();
    buf.put(&[0u8;2048][..]);

    upgrade(&mut io, &mut buf)?;
    ws(io, buf)?;
    Ok(())
}

const HTTP_REQUEST: &[u8] = b"\
GET / HTTP/1.1\r\n\
Host: 127.0.0.1:3000\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Version: 13\r\n\
\r\n";

fn upgrade(io: &mut TcpStream, buf: &mut BytesMut) -> Result<()> {
    io.write_all(HTTP_REQUEST)?;
    let read = io.read(buf)?;
    if read == 0 {
        return Ok(());
    }
    assert!(buf[..read].ends_with(b"\r\n\r\n"));
    buf.clear();
    Ok(())
}

fn ws(mut io: TcpStream, mut _buf: BytesMut) -> Result<()> {
    println!("[WS] Open");

    io.write_all(&[0x81,0x85,0x37,0xFA,0x21,0x3D,0x7F,0x9F,0x4D,0x51,0x58])?;

    let mut buf = [0u8;2048];
    let read = io.read(&mut buf)?;
    let read = &buf[..read];
    println!("[WS] Recv: {:?}",Bytes::copy_from_slice(read));

    io.write_all(&[0x9])?;

    let mut buf = [0u8;2048];
    let read = io.read(&mut buf)?;
    let read = &buf[..read];
    println!("[WS] Recv: {:?}",Bytes::copy_from_slice(read));

    Ok(())
}

