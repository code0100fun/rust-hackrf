#![feature(phase)]
#![feature(macro_rules)]

extern crate hackrf;
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;
extern crate libc;

use docopt::Docopt;
use hackrf::{HackRF};
use std::io::{File, Truncate, ReadWrite};
use std::os::unix::AsRawFd;
use libc::{setvbuf, _IOFBF};
use libc::funcs::posix88::stdio::fdopen;
use std::ptr;
use std::thread;

docopt!(Args deriving Show, "
Usage:
       hackrf_transfer -r <rx-filename> [-f <frequency>] [-s <sample-rate>] [-g <rx-vga>] [-l <rx-lna>] [-n <num-samples>] [-b <bb-filter>]
       hackrf_transfer (--help | --version)

Options:
    -h, --help                       Show this message.
    --version                        Show the version of hackrf_transfer.
    -r, --rx-filename <rx-filename>  Receive data into file.
    -f, --frequency <frequency>      Frequency in Hz.
    -s, --sample-rate <sample-rate>  Sample rate in Hz (8/10/12.5/16/20MHz, default 8).
    -g, --rx-vga <gain-db>           RX VGA (baseband) gain, 0-62dB, 2dB steps.
    -l, --rx-lna <gain-db>           RX LNA (IF) gain, 0-40dB, 8dB steps.
    -n, --num-samples <num-samples>  Number of samples to transfer (default is unlimited).
    -b, --bb-filter <bandwidth-hz>   Set baseband filter bandwidth in MHz.\n\tPossible values: 1.75/2.5/3.5/5/5.5/6/7/8/9/10/12/14/15/20/24/28MHz, default < sample_rate_hz.
",  flag_sample_rate: Option<u32>,
    flag_bb_filter: Option<u32>,
    flag_rx_vga: Option<u32>,
    flag_rx_lna: Option<u32>,
    flag_num_samples: Option<u32>,
);

fn open_file(filename: &str) -> File {
    let p = Path::new(filename);
    let file = match File::open_mode(&p, Truncate, ReadWrite) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
    let fd = file.as_raw_fd();
    let c95_file = unsafe { fdopen(fd, "w+".to_c_str().as_ptr()) };
    let null: *mut i8 = ptr::null_mut();
    let result = unsafe { setvbuf(c95_file, null, _IOFBF, 8*1024) };
    if result != 0 {
        panic!("setvbuf() failed: {}", result);
    }
    file
}

fn get_param<T>(value:Option<T>, default:T) -> T {
    match value {
        Some(v) => v,
        None => default
    }
}

fn set_param<T:Copy>(value:T, result:|T| -> Result<i32,String>, status:|T|) {
    match result(value) {
        Err(error) => panic!("{}", error),
        Ok(_) => status(value)
    }
}

fn setup_params(hackrf:&mut HackRF, args: &Args) {
    set_param(get_param(args.flag_sample_rate, 1000000),
        |v| { hackrf.set_sample_rate(v) },
        |v| { println!("Sample rate set to: {} Hz", v); }
    );

    // TODO - less than sample rate
    set_param(get_param(args.flag_bb_filter, 1750000),
        |v| { hackrf.set_baseband_filter_bandwidth(v) },
        |v| { println!("Baseband filter bandwidth set to: {} Hz", v); }
    );

    set_param(get_param(args.flag_rx_vga, 20),
        |v| { hackrf.set_vga_gain(v) },
        |v| { println!("VGA gain set to: {} dB", v); }
    );

    set_param(get_param(args.flag_rx_lna, 8),
        |v| { hackrf.set_lna_gain(v) },
        |v| { println!("LNA gain set to: {} dB", v); }
    );
}

fn bytes_to_write(bytes_rec:uint, bytes_left:uint, limit_samples:bool) -> uint {
    if limit_samples {
        let left = if bytes_left > bytes_rec {
            bytes_rec
        }else{
            bytes_left
        };
        left
    }else{
        bytes_rec
    }
}

fn read_into_file(rx:Receiver<Vec<u8>>, mut file:File, num_samples:u32) {
    let mut bytes_left = (num_samples * 2) as uint;
    let limit_samples = num_samples > 0;

    let _ = thread::Builder::new().name("writeFile".to_string()).spawn(move || {
        loop {
            let mut bytes = rx.recv();
            let to_write = bytes_to_write(bytes.len(), bytes_left, limit_samples);
            bytes_left -= to_write;
            bytes.truncate(to_write);
            match file.write(bytes.as_slice()) {
                Err(message) => { panic!("Failed to write to output: {}", message); },
                Ok(_) => {}
            }
            if limit_samples && bytes_left <= 0 {
                break;
            }
        }
        return 0i32;
    });
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    match HackRF::new() {
        Err(error) => println!("{}", error),
        Ok(mut hackrf) => {
            if hackrf.found {
                match hackrf.open() {
                    Err(error) => println!("{}", error),
                    Ok(_) => {
                        let num_samples = get_param(args.flag_num_samples, 0);
                        let file = open_file(args.flag_rx_filename.as_slice());
                        setup_params(&mut hackrf, &args);

                        let (_, rx) = hackrf.start();
                        read_into_file(rx, file, num_samples);
                    }
                }
            }
        }
    }
}
