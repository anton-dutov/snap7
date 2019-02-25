#[macro_use]
extern crate bitflags;
extern crate snap7_sys;

use snap7_sys::*;
use std::ffi::CString;
use std::os::raw::{
    c_int,
    c_char,
    c_void,
};

#[derive(Debug)]
struct Client {
    handle: S7Object,
    req_len: usize,
    neg_len: usize,
}

impl Client {
    pub fn new() -> Self {
        Self {
            handle: unsafe { Cli_Create() },
            req_len: 0,
            neg_len: 0,
        }
    }

    pub fn connect(&mut self) {

        let mut req: c_int = 0;
        let mut neg: c_int = 0;

        unsafe {
            Cli_ConnectTo(self.handle, CString::new("192.168.0.150").unwrap().as_ptr(), 0, 2);

            Cli_GetPduLength(self.handle, &mut req, &mut neg);

            self.req_len = req as usize;
            self.neg_len = neg as usize;
        }
    }


    pub fn read(&self, num: u32, start: u32, size: u32) -> Result<Vec<u8>, String> {

        let mut buf = Vec::<u8>::new();

        buf.resize(size as usize, 0);

        let res;
        unsafe {
            res = Cli_DBRead(
                self.handle,
                num as c_int,
                start as c_int,
                size as c_int,
                buf.as_mut_ptr() as *mut c_void
            ) as i32;

        }


        if res == 0 {
            Ok(buf)
        } else {
            Err(String::from(error_text(res)))
        }
    }

    pub fn close(&mut self) {

        unsafe {
            Cli_Disconnect(self.handle);
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {

        self.close();

        unsafe {
            Cli_Destroy(&mut self.handle);
        }
    }
}


fn main() {
    let mut client = Client::new();

    client.connect();

    println!("{:#?}", client);

    loop {
        println!("{:#?}", client.read(1420, 0, 20));
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}


pub fn error_text(code: i32) -> String {
    let mut err = Vec::<u8>::new();

        err.resize(1024, 0);

        unsafe {
            Cli_ErrorText(code as c_int, err.as_mut_ptr() as *mut c_char, err.len() as c_int);
        }

        if let Some(i) = err.iter().position(|&r| r == 0) {
            err.truncate(i);
        }

        let err = unsafe {
            std::str::from_utf8_unchecked(&err)
        };

        err.to_owned()
}


struct CtlRecord {
    plc_counter: u64,
    ctl_counter: u64,
}
