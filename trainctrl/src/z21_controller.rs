use std::net::UdpSocket;

use crate::{utils::primitives::Primitives, z21_command_receive::Z21CommandReceive, z21_command_send::Z21CommandSend};

#[derive(Debug, Clone)]
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
        let telegram = Z21CommandSend::LanGetSerialNumber.create_telegram();
        socket.send_to(&telegram, &self.ctrl_addr)?;

        let mut receive = [0; 8];
        socket.recv(&mut receive)?;
        let data = Z21CommandReceive::interprete_telegram(&receive);
        println!("serial: {:?}", &data);

        println!("send logoff");
        let telegram = Z21CommandSend::LanLogoff.create_telegram();
        socket.send_to(&telegram, &self.ctrl_addr)?;

        Ok(())
    }
}