use crate::{
    println,
    print, 
};

//Rust wrappers for C
///NOTE: ONLY CALL THIS FROM C AS RUST PANICS!1!
#[no_mangle]
pub unsafe extern "C" fn Cprintln(string: *const u8){
    let mut i = 0;
    while *string.offset(i) != 0{
        print!("{}", *string.offset(i) as char);
        i += 1;
    }
    println!();
}

pub fn bytes_to_kilobytes(bytes: u64) -> u64{
    bytes / 1000
}

pub fn kilobytes_to_megabytes(kilobytes: u64) -> u64{
    kilobytes / 1000
}

pub fn kilobytes_to_bytes(kilobytes: u64) -> u64{
    kilobytes * 1000
}

pub fn bytes_to_megabytes(bytes: u64) -> u64{
    (bytes_to_kilobytes(bytes)) / 1000
}

pub fn bytes_to_gigabytes(bytes: u64) -> u64{
    bytes_to_megabytes(bytes) / 1000
}

pub fn megabytes_to_bytes(megabytes: u64) -> u64{
    (megabytes * 1000) * 1000
}

