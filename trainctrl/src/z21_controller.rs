use std::net::UdpSocket;

use crate::utils::primitives::Primitives;

pub struct Z21Controller {
    bind_addr: String,
    ctrl_addr: String,
}

impl Z21Controller {

    pub fn new(bind_addr: &str, ctrl_addr: &str) -> Self {
        Self { 
            bind_addr: bind_addr.to_string(), 
            ctrl_addr: ctrl_addr.to_string(), 
        }
    }

    pub fn com_start(&self) -> std::io::Result<()> {

        let socket = UdpSocket::bind(&self.bind_addr)?;

        println!("send serial");
        let mut telegram = [0;4];
        telegram[0] = 0x04;
        telegram[2] = 0x1A;

        socket.send_to(&telegram, &self.ctrl_addr)?;

        let mut receive = [0; 12];
        socket.recv(&mut receive)?;
        let mut r = 4..8;
        let serial = Primitives::get_u32(&receive, &mut r); // 267734

        println!("serial: {:?}", &serial);

        println!("send logoff");
        let mut telegram = [0;4];
        telegram[0] = 0x04;
        telegram[2] = 0x30;

        socket.send_to(&telegram, &self.ctrl_addr)?;

        Ok(())
    }
}