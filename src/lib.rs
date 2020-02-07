use std::io;
use std::io::prelude::*;
use std::io::BufReader;

use std::net::TcpStream;

pub struct Client {
    name: String,
    connection: TcpStream,
}

impl Client {
    pub fn start(addr: &str, name: &str) -> io::Result<Client> {
        let mut stream = TcpStream::connect(addr)?;
        println!("Connected to {}", &stream.peer_addr()?);

        let name_line = format!("{}\n", name);
        stream.write_all(name_line.as_bytes())?;

        Ok(Client {
            name: name.into(),
            connection: stream
        })
    }

    pub fn read_line(&self) -> io::Result<String> {
        let mut line = String::new();

        let mut buf_reader = BufReader::new(&self.connection);
        buf_reader.read_line(&mut line)?;

        Ok(line)
    }

    pub fn write_msg(&self, msg: &str) -> io::Result<()> {
        (&self.connection).write_all(msg.as_bytes())
    }
}
