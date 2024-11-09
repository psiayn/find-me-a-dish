use std::io::{self, Write};
use std::time::Duration;

use clap::{value_parser, Arg, Command};

pub fn find_fridge_open() {
    let matches = Command::new("Serialport Example - Receive Data")
        .about("Reads data from a serial port and echoes it to stdout")
        .disable_version_flag(true)
        .arg(
            Arg::new("port")
                .help("The device path to a serial port")
                .use_value_delimiter(false)
                .required(true),
        )
        .arg(
            Arg::new("baud")
                .help("The baud rate to connect at")
                .use_value_delimiter(false)
                .required(true)
                .value_parser(value_parser!(u32)),
        )
        .get_matches();

    let port_name = matches.get_one::<String>("port").unwrap();
    let baud_rate = *matches.get_one::<u32>("baud").unwrap();
    println!("port_name: {port_name}");
    println!("baud_rate: {baud_rate}");

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        io::stdout().write_all(&serial_buf[..t]).unwrap();
                        io::stdout().flush().unwrap();
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(_) => (),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
