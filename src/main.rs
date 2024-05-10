use encoding_rs::GBK;
use local_encoding::Encoder;
use local_encoding::Encoding;
use std::env::current_dir;
use std::ffi::CString;
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Threading::*;
use windows::Win32::System::IO::*;

use crate::ParamValue::*;

fn main() -> Result<()> {
    let filename = current_dir().unwrap();
    let filename = filename.join("信息.txt");

    let mut string = filename.as_path().to_str().unwrap().to_owned();

    string.push('\0');
    let string1 = CString::from_vec_with_nul(string.as_bytes().to_vec()).unwrap();

    let encode = Encoding::ANSI
        .to_bytes("E:\\Rust\\example-ol\\信息.txt\0")
        .unwrap();
    let string2 = CString::from_vec_with_nul(encode).unwrap();

    let mut val = filename.as_os_str().as_encoded_bytes().to_vec();

    val.push(0);
    let string3 = CString::from_vec_with_nul(val).unwrap();

    let (encode, _, _) = GBK.encode("E:\\Rust\\example-ol\\信息.txt\0");
    let string2 = CString::from_vec_with_nul(encode.to_vec()).unwrap();

    println!("try to reading file {filename:?} --> ");
    println!("1 `{string1:?}`");
    println!("1 `{string2:?}`");
    println!("1 `{string3:?}`");

    let file = Owned(unsafe {
        CreateFileA(
            PCSTR(string2.as_ptr().cast()),
            FILE_GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            OPEN_EXISTING,
            FILE_FLAG_OVERLAPPED,
            None,
        )?
    });
    let mut overlapped = OVERLAPPED {
        Internal: 0,
        InternalHigh: 0,
        Anonymous: OVERLAPPED_0 {
            Anonymous: OVERLAPPED_0_0 {
                Offset: 9,
                OffsetHigh: 0,
            },
        },
        hEvent: unsafe { CreateEventA(None, true, false, None)? },
    };
    let mut buffer: [u8; 12] = Default::default();

    if let Err(e) = unsafe { ReadFile(file.abi(), Some(&mut buffer), None, Some(&mut overlapped)) }
    {
        assert_eq!(e.code(), ERROR_IO_PENDING.into());
    }

    unsafe {
        WaitForSingleObject(overlapped.hEvent, 2000);
    }
    let mut byte_copied = 0;

    unsafe {
        GetOverlappedResult(file.abi(), &overlapped, &mut byte_copied, false)?;
    }
    assert_eq!(byte_copied, 12);

    println!("--:> `{}`", String::from_utf8_lossy(&buffer));

    Ok(())
}
