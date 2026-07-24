mod z21_controller;
mod utils;

use crate::z21_controller::Z21Controller;

pub fn main() {
//    trainctrl_tracks::main();
    let result = udp();
    dbg!(&result);
}

fn udp() -> std::io::Result<()> {
    {
        let z21 = Z21Controller::new("0.0.0.0:0", "192.168.0.111:21105");
        z21.com_start()?;

    } // the socket is closed here
    Ok(())
}