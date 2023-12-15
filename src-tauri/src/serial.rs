use std::str;
use std::sync::{Arc, Mutex};
use std::thread::sleep;

use serial2::{self, SerialPort};

pub type Callback = Arc<Mutex<dyn FnMut(Vec<u32>) + Send + Sync>>;

#[derive(Debug)]
pub struct Serial {
    is_open: Mutex<bool>,
}

impl Serial {
    pub fn list_ports() -> Vec<String> {
        let mut result = vec![];

        match serial2::SerialPort::available_ports() {
            Err(e) => {
                eprintln!("Failed to enumerate serial ports: {}", e);
            }
            Ok(ports) => {
                for port in ports {
                    let p: String = format!("{}", port.display());
                    result.push(p);
                }
            }
        }
        result
    }

    pub fn open(port: &str, baud: u32, read_timeout_ms: u64, read_cb: Callback) -> Self {
        // Open the port
        let mut serial = SerialPort::open(port, baud).expect("Unable to open port");
        if read_timeout_ms > 0 {
            let timeout = std::time::Duration::from_millis(read_timeout_ms);
            serial
                .set_read_timeout(timeout)
                .expect("Unable to set read timeout");
        }

        let is_open: Mutex<bool> = true.into();

        // Create the comms channels

        // Start the thread
        let mut buf: [u8; 1024] = [0; 1024];
        std::thread::spawn(move || loop {
            // Read data and send it back via the callback
            match serial.read(&mut buf) {
                Ok(0) => (),
                Ok(n) => {
                    let cb = &mut read_cb.lock().unwrap();

                    let string_buf = match std::str::from_utf8(&buf[..n]) {
                        Ok(s) => s,
                        Err(e) => {
                            eprintln!("Error: Failed to read from port: {:?}", e);
                            ""
                        }
                    };

                    let numbers: Vec<u32> =
                        string_buf.lines().flat_map(|s| s.parse::<u32>()).collect();
                    sleep(std::time::Duration::from_micros(1));
                    cb(numbers);
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                Err(e) => {
                    eprintln!("Error: Failed to read from port: {:?}", e);
                    break;
                }
            };

            // Close the thread if we have to.
            {
                let remain_open = is_open.lock().unwrap();
                if !*remain_open {
                    break;
                }
            }
        });

        Self {
            is_open: true.into(),
        }
    }

    pub fn close(&self) {
        let mut is_open = self.is_open.lock().unwrap();
        *is_open = false;
    }
}
