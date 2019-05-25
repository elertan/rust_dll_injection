#[cfg(windows)] extern crate winapi;
use std::io::Error;
use winapi::shared::minwindef::*;

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}
#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

fn init() {
    print_message("Hello, world!").unwrap();
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID)
    -> BOOL {

        const DLL_PROCESS_ATTACH: DWORD = 1;
        const DLL_PROCESS_DETACH: DWORD = 0;

        match call_reason {
            DLL_PROCESS_ATTACH => init(),
            DLL_PROCESS_DETACH => (),
            _ => ()
        }

        return TRUE;
}
