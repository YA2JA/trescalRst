use std::ffi::CString;
use std::io::{BufRead, BufReader, Write};
use visa_rs::prelude::*;

pub struct Communication
{
    rm: DefaultRM,
    address:u8
}

impl Communication
{
    pub fn new(address:u8) -> Self
    {
        let rm: DefaultRM = DefaultRM::new().expect("Error can't init GPIB comminication");
        Communication {rm: rm, address:address}
    }

    pub fn get(&self, message:&[u8]) -> String
    {
        let expr = CString::new(format!("?*{}?*INSTR", &self.address)).unwrap().into();
        let rsc = (&self.rm).find_res(&expr).expect("Oupss...");
        let instr: Instrument = (&self.rm).open( &rsc,
                                        AccessMode::NO_LOCK, 
                                        TIMEOUT_IMMEDIATE).expect("Oupss...");
        (&instr).write_all(message).map_err(io_to_vs_err).expect("Oupss...");
        let mut buf_reader = BufReader::new(&instr);
        let mut buf = String::new();
        buf_reader.read_line(&mut buf).map_err(io_to_vs_err).expect("Oupss...");
        return buf;
    }

    pub fn set(&self,  message:&[u8])
    {
        let expr = CString::new(format!("?*{}?*INSTR", &self.address)).unwrap().into();
        let rsc = (&self.rm).find_res(&expr).expect("Oupss...");
        let instr: Instrument = (&self.rm).open( &rsc,
                                        AccessMode::NO_LOCK, 
                                        TIMEOUT_IMMEDIATE).expect("Oupss...");
        (&instr).write_all(message).map_err(io_to_vs_err).expect("Oupss...");
    }
    
}