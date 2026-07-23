pub fn main() {
//    trainctrl_tracks::main();
    let result = udp();
    dbg!(&result);
}

use std::net::UdpSocket;

fn udp() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:0")?;

        println!("send serial");
        let mut telegram = [0;4];
        telegram[0] = 0x04;
        telegram[2] = 0x10;

        socket.send_to(&telegram, "192.168.0.111:21105")?;

        let mut receive = [0; 8];
        socket.recv(&mut receive)?;

        println!("serial: {:?}", &receive);

        println!("send logoff");
        let mut telegram = [0;4];
        telegram[0] = 0x04;
        telegram[2] = 0x30;

        socket.send_to(&telegram, "192.168.0.111:21105")?;

    } // the socket is closed here
    Ok(())
}