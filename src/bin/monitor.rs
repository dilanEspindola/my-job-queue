use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

fn main() -> std::io::Result<()> {
    let socket_path = "test.sock";
    let mut stream = UnixStream::connect(socket_path)?;
    stream.write_all(b"helloo")?;

    let mut response = String::new();

    stream.read_to_string(&mut response)?;

    println!("Received response: {}", response);

    Ok(())
}
