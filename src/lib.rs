use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};
use bytes::{BufMut, BytesMut};

use crate::frame::{Frame, Payload};

mod mask;
mod frame;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn client() -> Result<()> {
    let mut io = TcpStream::connect("127.0.0.1:3000")?;
    let mut buf = BytesMut::with_capacity(2048);

    upgrade_client(&mut io, &mut buf)?;
    ws_send(&mut io, Frame::text(Payload::Borrowed(b"Rust Client")), true)?;
    // println!("[MSG] {:?}",ws_recv(&mut io, buf)?);
    Ok(())
}

pub fn server() -> Result<()> {
    let io = TcpListener::bind("127.0.0.1:3000")?;

    loop {
        let mut buf = BytesMut::with_capacity(2048);

        let (mut io,_) = match io.accept() {
            Ok(ok) => ok,
            Err(err) => {
                eprintln!("{err}");
                continue;
            },
        };

        upgrade_server(&mut io, &mut buf)?;
        println!("[MSG] {:?}",ws_recv(&mut io, buf)?);
        ws_send(&mut io, Frame::text(Payload::Borrowed(b"Rust Server")), false)?;
    }
}

const HTTP_REQUEST: &[u8] = b"\
GET / HTTP/1.1\r\n\
Host: 127.0.0.1:3000\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Version: 13\r\n\
\r\n";

const HTTP_RESPONSE: &[u8] = b"\
HTTP/1.1 101 Switching Protocols\r\n\
Upgrade: websocket\r\n\
Connection: Upgrade\r\n\
Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n\
Date: Mon, 16 Jun 2025 00:14:32 GMT\r\n\
\r\n";

fn upgrade_client(io: &mut TcpStream, buf: &mut BytesMut) -> Result<()> {
    io.write_all(HTTP_REQUEST)?;
    let uninit = buf.chunk_mut();
    let read = io.read(unsafe { std::mem::transmute::<&mut [std::mem::MaybeUninit<u8>], &mut [u8]>(uninit.as_uninit_slice_mut()) })?;
    if read == 0 {
        return Ok(());
    }
    unsafe { buf.advance_mut(read) };

    assert!(buf[..read].ends_with(b"\r\n\r\n"));
    // dbg!(buf.split_to(read));
    buf.clear();
    Ok(())
}

fn upgrade_server(io: &mut TcpStream, buf: &mut BytesMut) -> Result<()> {
    let uninit = buf.chunk_mut();
    let read = io.read(unsafe { std::mem::transmute::<&mut [std::mem::MaybeUninit<u8>], &mut [u8]>(uninit.as_uninit_slice_mut()) })?;
    if read == 0 {
        return Ok(());
    }
    unsafe { buf.advance_mut(read) };

    assert!(buf[..read].ends_with(b"\r\n\r\n"));
    buf.clear();
    io.write_all(HTTP_RESPONSE)?;
    Ok(())
}

fn ws_send(io: &mut TcpStream, mut frame: Frame, is_mask: bool) -> Result<()> {
    // frame.writev(io).map_err(Into::into)

    if is_mask {
        frame.mask();
    }

    let mut buf = Vec::with_capacity(516);
    let frame = frame.write(&mut buf);
    io.write_all(frame).map_err(Into::into)
}

fn ws_recv(io: &mut TcpStream, mut buf: BytesMut) -> Result<BytesMut> {
    let uninit = buf.chunk_mut();
    let read = io.read(unsafe { std::mem::transmute::<&mut [std::mem::MaybeUninit<u8>], &mut [u8]>(uninit.as_uninit_slice_mut()) })?;
    if read == 0 {
        Err(io::Error::from(io::ErrorKind::InvalidData))?;
    }
    unsafe { buf.advance_mut(read) };

    Ok(buf.split_to(read))
}

