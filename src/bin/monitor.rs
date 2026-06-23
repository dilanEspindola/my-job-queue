use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() -> std::io::Result<()> {
    let argss = Args::parse();

    println!("{}", argss.name);

    let socket_path = "test.sock";
    let mut stream = UnixStream::connect(socket_path)?;
    stream.write_all(argss.name.as_bytes())?;

    let mut response = String::new();

    stream.read_to_string(&mut response)?;

    println!("Received response: {}", response);

    Ok(())
}
