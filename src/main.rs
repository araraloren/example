#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(unused)]
mod sqlite3 {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use crate::sqlite3::sqlite3_errmsg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db = null_mut();
    let name = CString::new("test.db")?;
    let rc = unsafe { sqlite3::sqlite3_open(name.as_ptr(), &mut db as *mut _) };

    if rc as u32 != sqlite3::SQLITE_OK {
        let msg = unsafe { sqlite3_errmsg(db) };
        panic!("Can not open database: {:?}", unsafe {
            &CStr::from_ptr(msg)
        });
    } else {
        println!("Open database successfully");
    }

    unsafe {
        dbg!(sqlite3::sqlite3_close(db));
    }

    Ok(())
}
