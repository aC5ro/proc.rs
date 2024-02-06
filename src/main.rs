use std::mem;
use std::ptr::{ null_mut , null };
use std::ffi::CString;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winbase::CREATE_NEW_CONSOLE;
use winapi::um::processthreadsapi::{ CreateProcessA, PROCESS_INFORMATION, STARTUPINFOA };

fn main() {
    println!("[*] Creating Process...");
    let command = CString::new("C:\\Windows\\System32\\notepad.exe").expect("Failed to create string.");
    let mut startup_info: STARTUPINFOA = unsafe { mem::zeroed() };
    startup_info.cb = mem::size_of::<STARTUPINFOA>() as u32;
    let mut process_info: PROCESS_INFORMATION = unsafe { mem::zeroed() };
    let success = unsafe {
        CreateProcessA(
            null(),
            command.as_ptr() as *mut i8,
            null_mut(),
            null_mut(),
            false as i32,
            CREATE_NEW_CONSOLE,
            null_mut(),
            null(),
            &mut startup_info,
            &mut process_info,
        )
    };

    if success != 0 {
        println!("[+] Process created successfully!");
        unsafe {
            CloseHandle(process_info.hProcess);
            CloseHandle(process_info.hThread);
        }
    } else {
        println!("[-] Failed to create process.")
    }
}
