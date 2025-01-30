#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

pub type LPARAM = isize;
pub type LRESULT = isize;
pub type WPARAM = usize;
pub type BOOL = i32;

//type UINT=u16;
pub type PCWSTR = *const u16;
pub type WNDCLASS_STYLES = u32;

pub type HWND = *mut core::ffi::c_void;

pub type HMODULE = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HICON = *mut core::ffi::c_void;
pub type HCURSOR = *mut core::ffi::c_void;
pub type HBRUSH = *mut core::ffi::c_void;
pub type HMENU = *mut core::ffi::c_void;

pub type SHOW_WINDOW_CMD = i32;
pub type WINDOW_STYLE = u32;
pub type WINDOW_EX_STYLE = u32;

pub const CS_OWNDC: WNDCLASS_STYLES = 32u32;
pub const CS_HREDRAW: WNDCLASS_STYLES = 2u32;
pub const CS_VREDRAW: WNDCLASS_STYLES = 1u32;

pub const IDC_ARROW: PCWSTR = 32512u16 as _;
pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = 13565952u32;
pub const WS_VISIBLE: WINDOW_STYLE = 268435456u32;
pub const CW_USEDEFAULT: i32 = -2147483648i32;

pub const SW_SHOW: SHOW_WINDOW_CMD = 5i32;

pub const WM_DESTROY: u32 = 2u32;

pub type WNDPROC = Option<unsafe extern "system" fn(param0: HWND, param1: u32,
 param2: WPARAM, param3: LPARAM) -> LRESULT>;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WNDCLASSW {
    pub style: WNDCLASS_STYLES,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: i32,
    pub cbWndExtra: i32,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: PCWSTR,
    pub lpszClassName: PCWSTR,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: u32,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: u32,
    pub pt: POINT,
}

windows_targets::link!("kernel32.dll" "system" fn GetModuleHandleW(lpmodulename : PCWSTR) ->  HMODULE);
windows_targets::link!("user32.dll" "system" fn CreateWindowExW(dwexstyle : WINDOW_EX_STYLE, 
  lpclassname : PCWSTR, lpwindowname : PCWSTR, dwstyle : WINDOW_STYLE, x : i32, y : i32, nwidth : i32, nheight : i32,
  hwndparent : HWND, hmenu : HMENU, hinstance : HINSTANCE, lpparam : *const core::ffi::c_void) -> HWND);
windows_targets::link!("user32.dll" "system" fn ShowWindow(hwnd : HWND, ncmdshow : SHOW_WINDOW_CMD) -> BOOL);
windows_targets::link!("user32.dll" "system" fn GetMessageW(lpmsg : *mut MSG, hwnd :HWND, wmsgfiltermin : u32, wmsgfiltermax : u32) -> BOOL);
windows_targets::link!("user32.dll" "system" fn TranslateMessage(lpmsg : *const MSG) -> BOOL);
windows_targets::link!("user32.dll" "system" fn DispatchMessageW(lpmsg : *const MSG) -> LRESULT);

windows_targets::link!("user32.dll" "system" fn LoadCursorW(hinstance : HINSTANCE, lpcursorname : PCWSTR) -> HCURSOR);
windows_targets::link!("user32.dll" "system" fn RegisterClassW(lpwndclass : *const WNDCLASSW) -> u16);
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> u32);
windows_targets::link!("user32.dll" "system" fn PostQuitMessage(nexitcode : i32));
windows_targets::link!("user32.dll" "system" fn DefWindowProcW(hwnd : HWND, msg : u32, wparam :  WPARAM, lparam : LPARAM) -> LRESULT);

fn main() {
    
    
    let app_name = to_wstring("Rust Meets Windows");

    let h_instance = unsafe { GetModuleHandleW(null_mut()) };

    let wnd_class = WNDCLASSW {
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        hInstance: h_instance,
        lpszClassName: app_name.as_ptr(),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hIcon: null_mut(),
        hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
        hbrBackground: null_mut(),
        lpszMenuName: null_mut(),
    };

    let class_atom = unsafe { RegisterClassW(&wnd_class) };

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_atom as *const u16,
            app_name.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            h_instance,
            null_mut(),
        )
    };

    if hwnd.is_null() {
        panic!("Failed to create window.");
    }

    unsafe { ShowWindow(hwnd, SW_SHOW) };

    let mut msg = MSG {
        hwnd: null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: Default::default(),
    };

    while unsafe { GetMessageW(&mut msg, null_mut(), 0, 0) } != 0 {
        unsafe {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

fn to_wstring(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(once(0))
        .collect()
}