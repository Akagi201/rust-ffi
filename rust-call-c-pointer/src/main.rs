mod mytime;
use mytime::*;
use core::ffi::CStr;

fn main() {
    let mut sometime  = StructTM {
        tm_year: 1,
        tm_mon: 1,
        tm_mday: 1,
        tm_hour: 1, 
        tm_min: 1,
        tm_sec: 1,
        tm_isdst: -1,
        tm_wday: 1,
        tm_yday: 1
    };

    unsafe {
        let c_ptr = &mut sometime; // raw pointer

        // make the call, convert and then own
        // the returned C string
        let char_ptr = asctime(c_ptr);
        let c_str = CStr::from_ptr(char_ptr);
        println!("{:#?}", c_str.to_str());
   
        let utc = mktime(c_ptr); 
        println!("{}", utc);
    }
}
