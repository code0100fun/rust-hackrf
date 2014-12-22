/* automatically generated by rust-bindgen */
extern crate libc;
extern crate core;

use libc::{c_char, c_uchar, c_double, c_int, c_uint, c_void, uint8_t, uint16_t, uint32_t, uint64_t};

pub type EnumHackrfError = c_int;
pub const HACKRF_SUCCESS: c_int = 0;
pub const HACKRF_TRUE: c_int = 1;
pub const HACKRF_ERROR_INVALID_PARAM: c_int = -2;
pub const HACKRF_ERROR_NOT_FOUND: c_int = -5;
pub const HACKRF_ERROR_BUSY: c_int = -6;
pub const HACKRF_ERROR_NO_MEM: c_int = -11;
pub const HACKRF_ERROR_LIBUSB: c_int = -1000;
pub const HACKRF_ERROR_THREAD: c_int = -1001;
pub const HACKRF_ERROR_STREAMING_THREAD_ERR: c_int = -1002;
pub const HACKRF_ERROR_STREAMING_STOPPED: c_int = -1003;
pub const HACKRF_ERROR_STREAMING_EXIT_CALLED: c_int = -1004;
pub const HACKRF_ERROR_OTHER: c_int = -9999;
pub type EnumHackrfBoardId = uint8_t;
pub const BOARD_ID_JELLYBEAN: c_uint = 0;
pub const BOARD_ID_JAWBREAKER: c_uint = 1;
pub const BOARD_ID_HACKRF_ONE: c_uint = 2;
pub const BOARD_ID_INVALID: c_uint = 255;
pub type EnumRfPathFilter = c_uint;
pub const RF_PATH_FILTER_BYPASS: c_uint = 0;
pub const RF_PATH_FILTER_LOW_PASS: c_uint = 1;
pub const RF_PATH_FILTER_HIGH_PASS: c_uint = 2;

#[deriving(Copy)]
pub enum StructHackrfDevice { }
pub type HackrfDevice = StructHackrfDevice;

#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct hackrf_transfer {
    pub device: *mut HackrfDevice,
    pub buffer: *mut uint8_t,
    pub buffer_length: c_int,
    pub valid_length: c_int,
    pub rx_ctx: *mut c_void,
    pub tx_ctx: *mut c_void,
}
pub type HackrfTransferT = hackrf_transfer;

#[repr(C)]
#[deriving(Copy)]
pub struct read_partid_serialno {
    pub part_id: [uint32_t, ..2u],
    pub serial_no: [uint32_t, ..4u],
}
pub type ReadPartidSerialnoT = read_partid_serialno;

pub type HackrfSampleBlockCbFn =
    ::std::option::Option<extern "C" fn(arg1: *mut HackrfTransferT)
                              -> c_int>;

#[link(name = "hackrf")]
extern "C" {
    pub fn hackrf_init() -> c_int;
    pub fn hackrf_exit() -> c_int;
    pub fn hackrf_open(device: & *mut HackrfDevice) -> c_int;
    pub fn hackrf_close(device: *mut HackrfDevice) -> c_int;
    pub fn hackrf_start_rx(device: *mut HackrfDevice, callback: HackrfSampleBlockCbFn, rx_ctx: *mut c_void) -> c_int;
    pub fn hackrf_stop_rx(device: *mut HackrfDevice) -> c_int;
    pub fn hackrf_start_tx(device: *mut HackrfDevice, callback: HackrfSampleBlockCbFn, tx_ctx: *mut c_void) -> c_int;
    pub fn hackrf_stop_tx(device: *mut HackrfDevice) -> c_int;
    pub fn hackrf_is_streaming(device: *mut HackrfDevice) -> c_int;
    pub fn hackrf_max2837_read(device: *mut HackrfDevice, register_number: uint8_t, value: *mut uint16_t) -> c_int;
    pub fn hackrf_max2837_write(device: *mut HackrfDevice, register_number: uint8_t, value: uint16_t) -> c_int;
    pub fn hackrf_si5351c_read(device: *mut HackrfDevice, register_number: uint16_t, value: *mut uint16_t) -> c_int;
    pub fn hackrf_si5351c_write(device: *mut HackrfDevice, register_number: uint16_t, value: uint16_t) -> c_int;
    pub fn hackrf_set_baseband_filter_bandwidth(device: *mut HackrfDevice, bandwidth_hz: uint32_t) -> c_int;
    pub fn hackrf_rffc5071_read(device: *mut HackrfDevice, register_number: uint8_t, value: *mut uint16_t) -> c_int;
    pub fn hackrf_rffc5071_write(device: *mut HackrfDevice, register_number: uint8_t, value: uint16_t) -> c_int;
    pub fn hackrf_spiflash_erase(device: *mut HackrfDevice) -> c_int;
    pub fn hackrf_spiflash_write(device: *mut HackrfDevice, address: uint32_t, length: uint16_t, data: *mut c_uchar) -> c_int;
    pub fn hackrf_spiflash_read(device: *mut HackrfDevice, address: uint32_t, length: uint16_t, data: *mut c_uchar) -> c_int;
    pub fn hackrf_cpld_write(device: *mut HackrfDevice, data: *mut c_uchar, total_length: c_uint) -> c_int;
    pub fn hackrf_board_id_read(device: *mut HackrfDevice, value: *mut uint8_t) -> c_int;
    pub fn hackrf_version_string_read(device: *mut HackrfDevice, version: *mut c_char, length: uint8_t) -> c_int;
    pub fn hackrf_set_freq(device: *mut HackrfDevice, freq_hz: uint64_t) -> c_int;
    pub fn hackrf_set_freq_explicit(device: *mut HackrfDevice, if_freq_hz: uint64_t, lo_freq_hz: uint64_t, path: EnumRfPathFilter) -> c_int;
    pub fn hackrf_set_sample_rate_manual(device: *mut HackrfDevice, freq_hz: uint32_t, divider: uint32_t) -> c_int;
    pub fn hackrf_set_sample_rate(device: *mut HackrfDevice, freq_hz: c_double) -> c_int;
    pub fn hackrf_set_amp_enable(device: *mut HackrfDevice, value: uint8_t) -> c_int;
    pub fn hackrf_board_partid_serialno_read(device: *mut HackrfDevice, read_partid_serialno: *mut ReadPartidSerialnoT) -> c_int;
    pub fn hackrf_set_lna_gain(device: *mut HackrfDevice, value: uint32_t) -> c_int;
    pub fn hackrf_set_vga_gain(device: *mut HackrfDevice, value: uint32_t) -> c_int;
    pub fn hackrf_set_txvga_gain(device: *mut HackrfDevice, value: uint32_t) -> c_int;
    pub fn hackrf_set_antenna_enable(device: *mut HackrfDevice, value: uint8_t) -> c_int;
    pub fn hackrf_error_name(errcode: EnumHackrfError) -> *const c_char;
    pub fn hackrf_board_id_name(board_id: EnumHackrfBoardId) -> *const c_char;
    pub fn hackrf_filter_path_name(path: EnumRfPathFilter) -> *const c_char;
    pub fn hackrf_compute_baseband_filter_bw_round_down_lt(bandwidth_hz: uint32_t) -> uint32_t;
    pub fn hackrf_compute_baseband_filter_bw(bandwidth_hz: uint32_t) -> uint32_t;
}

#[allow(missing_copy_implementations)]
pub struct HackRF {
    device: *mut HackrfDevice,
    pub found: bool
}

impl HackRF {

    pub fn new() -> Result<HackRF, String> {
        let device: *mut HackrfDevice;
        let hackrf = HackRF { device: device, found: false };
        match hackrf.init() {
            Ok(_) => Ok(HackRF { device: device, found: true }),
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

    pub fn open(&self) -> Result<i32, String> {
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

    pub fn part_id(&self) -> Result<[uint32_t, ..2u], String> {
        let mut info = ReadPartidSerialnoT { part_id: [0,0], serial_no: [0, 0, 0, 0] };
        let status = unsafe {
            hackrf_board_partid_serialno_read(self.device, &mut info)
        };
        match status {
            HACKRF_SUCCESS => Ok(info.part_id),
            _ => Err("Failed to get part ID".to_string())
        }
    }

    pub fn serial_no(&self) -> Result<[uint32_t, ..4u], String> {
        let mut info = ReadPartidSerialnoT { part_id: [0,0], serial_no: [0, 0, 0, 0] };
        let status = unsafe {
            hackrf_board_partid_serialno_read(self.device, &mut info)
        };
        match status {
            HACKRF_SUCCESS => Ok(info.serial_no),
            _ => Err("Failed to get serial number".to_string())
        }
    }

}
