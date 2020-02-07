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

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn start_client(addr: *const c_char, name: *const c_char) -> *mut Client {
    let addr = unsafe { CStr::from_ptr(addr).to_str().unwrap() };
    let name = unsafe { CStr::from_ptr(name).to_str().unwrap() };

    let client = Client::start(addr, name).unwrap();

    let boxed_client = Box::new(client);
    Box::into_raw(boxed_client)
}

#[no_mangle]
pub extern "C" fn write_msg(c: &Client, msg: *const c_char) {
    let msg = unsafe { CStr::from_ptr(msg).to_str().unwrap() };

    c.write_msg(msg).unwrap();
}

#[no_mangle]
pub extern "C" fn read_line(c: &Client) -> *mut c_char {
    let line = c.read_line().unwrap();

    CString::new(line).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_client(client: *mut Client) {
    unsafe { Box::from_raw(client) };
}
