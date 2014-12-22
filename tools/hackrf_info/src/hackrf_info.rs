extern crate hackrf;

use hackrf::{HackRF};

pub fn main() {
    match HackRF::new() {
        Err(error) => println!("{}", error),
        Ok(hackrf) => {
            if hackrf.found {

                match hackrf.open() {
                    Err(error) => println!("{}", error),
                    Ok(_) => {
                        println!("Found HackRF board.");

                        match hackrf.board_id() {
                            Ok(board_id) => {
                                match hackrf.board_name() {
                                    Ok(board_name) => {
                                        println!("Board ID Number: {} ({})", board_id, board_name)
                                    },
                                    Err(error) => panic!(error)
                                }
                            },
                            Err(error) => panic!(error)
                        }

                        match hackrf.firmware_version() {
                            Ok(version) => println!("Firmware Version: {}", version),
                            Err(error) => panic!(error)
                        }

                        match hackrf.part_id() {
                            Ok(part_id) => println!("Part ID Number: 0x{0:08x} 0x{1:08x}", part_id[0], part_id[1]),
                            Err(error) => panic!(error)
                        }

                        match hackrf.serial_no() {
                            Ok(serial_no) => println!("Serial Number: 0x{0:08x} 0x{1:08x} 0x{2:08x} 0x{3:08x}", serial_no[0], serial_no[1], serial_no[2], serial_no[3]),
                            Err(error) => panic!(error)
                        }

                    },
                }

            }
        }
    }
}
