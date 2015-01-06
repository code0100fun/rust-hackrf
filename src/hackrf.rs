#![feature(globs)]
extern crate libc;
extern crate core;

use ffi::*;
use libc::{c_int, uint32_t, uint64_t};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::c_str::ToCStr;

mod ffi;

extern "C" fn rx_callback(transfer: *mut HackrfTransferT) -> i32 {
    let trans = unsafe { & *transfer };
    let hackrf_ptr = trans.rx_ctx;
    let sender: &Option<Sender<Vec<u8>>> = unsafe { std::mem::transmute(hackrf_ptr) };
    match sender {
        &Some(ref rx_send) => {
            let buffer = unsafe { std::vec::Vec::from_raw_buf(trans.buffer, trans.valid_length as uint) };
            let _ = rx_send.send(buffer);
        },
        &None => { println!("None"); }
    };
    return 0i32;
}

#[allow(missing_copy_implementations)]
pub struct HackRF {
    device: *mut HackrfDevice,
    pub found: bool,
    pub rx: Option<Sender<Vec<u8>>>,
    pub tx: Option<Receiver<Vec<u8>>>
}

impl HackRF {

    pub fn new() -> Result<HackRF, String> {
        let device: *mut HackrfDevice;
        let hackrf = HackRF { device: device, found: false, rx: None, tx: None };
        match hackrf.init() {
            Ok(_) => {
                let hackrf = HackRF { device: device, found: true, rx: None, tx: None };
                Ok(hackrf)
            },
            Err(error) => Err(error)
        }
    }

    fn init(&self) -> Result<i32, String> {
        let status = unsafe {
            hackrf_init()
        };
        match status {
            HACKRF_SUCCESS => Ok(0),
            _ => Err("HackRF not found".to_string())
        }
    }

    pub fn open(opened:|HackRF|) {
        match HackRF::new() {
            Err(error) => println!("{}", error),
            Ok(hackrf) => {
                if hackrf.found {
                    match hackrf.open_device() {
                        Err(error) => println!("{}", error),
                        Ok(_) => {
                            opened(hackrf);
                        }
                    }
                }
            }
        }
    }

    pub fn open_device(&self) -> Result<i32, String> {
        let status = unsafe {
            hackrf_open(&self.device)
        };
        match status {
            HACKRF_SUCCESS => Ok(0),
            _ => Err("Failed to open HackRF".to_string())
        }
    }

    pub fn board_id(&self) -> Result<u8, String> {
        let mut value = 0u8;
        let status = unsafe {
            hackrf_board_id_read(self.device, &mut value)
        };
        match status {
            HACKRF_SUCCESS => Ok(value),
            _ => Err("Failed to get board id".to_string())
        }
    }

    pub fn board_name(&self) -> Result<String, String> {
        match self.board_id() {
            Ok(board_id) =>  {
                let board_name_ptr = unsafe {
                    hackrf_board_id_name(board_id)
                };
                let board_name = unsafe { String::from_raw_buf(board_name_ptr as *const u8) };
                Ok(board_name)
            },
            Err(error) => Err(error)
        }
    }

    pub fn firmware_version(&self) -> Result<String, String> {
        let mut buffer = String::with_capacity(255 + 1).to_c_str();
        let status = unsafe {
            hackrf_version_string_read(self.device, buffer.as_mut_ptr(), 255)
        };
        match status {
            HACKRF_SUCCESS => Ok(buffer.to_string()),
            _ => Err("Failed to get device version".to_string())
        }
    }

    pub fn part_id(&self) -> Result<[uint32_t; 2u], String> {
        let mut info = ReadPartidSerialnoT { part_id: [0,0], serial_no: [0, 0, 0, 0] };
        let status = unsafe {
            hackrf_board_partid_serialno_read(self.device, &mut info)
        };
        match status {
            HACKRF_SUCCESS => Ok(info.part_id),
            _ => Err("Failed to get part ID".to_string())
        }
    }

    pub fn serial_no(&self) -> Result<[uint32_t; 4u], String> {
        let mut info = ReadPartidSerialnoT { part_id: [0,0], serial_no: [0, 0, 0, 0] };
        let status = unsafe {
            hackrf_board_partid_serialno_read(self.device, &mut info)
        };
        match status {
            HACKRF_SUCCESS => Ok(info.serial_no),
            _ => Err("Failed to get serial number".to_string())
        }
    }

    pub fn set_freq(&self, freq_hz: uint64_t) -> Result<c_int, String> {
        let status = unsafe {
            hackrf_set_freq(self.device, freq_hz)
        };
        match status {
            HACKRF_SUCCESS => Ok(HACKRF_SUCCESS),
            _ => Err("Failed to set frequency".to_string())
        }
    }

    pub fn set_sample_rate(&self, freq_hz: uint32_t) -> Result<c_int, String> {
        let status = unsafe {
            hackrf_set_sample_rate_manual(self.device, freq_hz, 1)
        };
        match status {
            HACKRF_SUCCESS => Ok(HACKRF_SUCCESS),
            _ => Err("Failed to set sample rate".to_string())
        }
    }

    pub fn set_baseband_filter_bandwidth(&self, bandwidth_hz: uint32_t) -> Result<c_int, String> {
        let status = unsafe {
            hackrf_set_baseband_filter_bandwidth(self.device, bandwidth_hz)
        };
        match status {
            HACKRF_SUCCESS => Ok(HACKRF_SUCCESS),
            _ => Err("Failed to set baseband filter bandwidth".to_string())
        }
    }

    pub fn set_vga_gain(&mut self, value: uint32_t) -> Result<c_int, String> {
        let status = unsafe {
            hackrf_set_vga_gain(self.device, value)
        };
        match status {
            HACKRF_SUCCESS => Ok(HACKRF_SUCCESS),
            _ => Err("Failed to set VGA gain".to_string())
        }
    }

    pub fn set_lna_gain(&mut self, value: uint32_t) -> Result<c_int, String> {
        let status = unsafe {
            hackrf_set_lna_gain(self.device, value)
        };
        match status {
            HACKRF_SUCCESS => Ok(HACKRF_SUCCESS),
            _ => Err("Failed to set LNA gain".to_string())
        }
    }

    pub fn start_rx(&mut self) -> Receiver<Vec<u8>> {
        let (rx_send, rx_rec) = channel::<Vec<u8>>();
        self.rx = Some(rx_send);
        unsafe {
            hackrf_start_rx(self.device, rx_callback, &self.rx);
        };
        return rx_rec;
    }

    pub fn start(&mut self) -> (Sender<Vec<u8>>, Receiver<Vec<u8>>) {
        let rx_rec = self.start_rx();
        let (tx_send, _) = channel::<Vec<u8>>();
        return (tx_send, rx_rec);
    }
}
