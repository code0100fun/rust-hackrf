#![allow(dead_code)]
extern crate libc;
extern crate core;

use libc::{c_char, c_uchar, c_double, c_int, c_uint, c_void, uint8_t, uint16_t, uint32_t, uint64_t};
use std::sync::mpsc::{Sender};

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

#[derive(Copy)]
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
#[derive(Copy)]
pub struct read_partid_serialno {
    pub part_id: [uint32_t; 2u],
    pub serial_no: [uint32_t; 4u],
}
pub type ReadPartidSerialnoT = read_partid_serialno;

#[repr(C)]
pub type HackrfTransferCallback = extern "C" fn(transfer: *mut HackrfTransferT) -> c_int;

#[repr(C)]
pub type HackrfTransferContext = Option<Sender<Vec<u8>>>;

#[repr(C)]
#[link(name = "hackrf")]
#[allow(dead_code)]
extern "C" {
    pub fn hackrf_init() -> c_int;
    pub fn hackrf_exit() -> c_int;
    pub fn hackrf_open(device: & *mut HackrfDevice) -> c_int;
    pub fn hackrf_close(device: *mut HackrfDevice) -> c_int;
    pub fn hackrf_start_rx(device: *mut HackrfDevice, callback: HackrfTransferCallback, rx: &HackrfTransferContext) -> c_int;
    pub fn hackrf_stop_rx(device: *mut HackrfDevice) -> c_int;
    pub fn hackrf_start_tx(device: *mut HackrfDevice, callback: HackrfTransferCallback, tx_ctx: *mut c_void) -> c_int;
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

