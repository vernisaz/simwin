//extern crate windows_targets;

fn main() {
    windows_targets::link!("kernel32.dll" "system" fn SetLastError(code: u32));
    windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> u32);
    
    unsafe {
        SetLastError(1235);
        assert_eq!(GetLastError(), 1235);
    }
    println!("test passed")
}