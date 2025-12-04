#![allow(non_snake_case)]
extern crate simwinapi;

use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;

use std::ptr::null_mut;
use simwinapi::*;

fn main() {
    
    let app_name = to_wstring("Rust using Windows API");

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
        hbrBackground: (COLOR_BACKGROUND+1) as _ , //null_mut(),
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

    unsafe { ShowWindow(hwnd, SW_SHOW) ; } 

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
        WM_PAINT => on_paint(hwnd),
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

unsafe fn on_paint(hWnd: HWND) -> LRESULT {
    
    let mut ps = PAINTSTRUCT{ hdc: null_mut(),
    fErase: 0,
    rcPaint: RECT{..Default::default()},
    fRestore: 0,
    fIncUpdate: 0,
    rgbReserved: [0; 32]};
    let hdc = BeginPaint(hWnd, &mut ps);
   
    SetTextColor(hdc, RGB(0xEF, 0xBD, 0x52));
    SetBkMode(hdc, TRANSPARENT as _);
    
    let mut start_y = 50;
    match File::open("..\\README.md") {
        Ok(file) =>  for line in BufReader::new(file).lines() {
            if let Ok(line) = line {
                let line = to_wstring(&line);
                TextOutW(hdc, 32, start_y, line.as_ptr(), line.len() as _);
                start_y += 21
            }
        }
        Err(err) => {
            SetTextColor(hdc, 0x003333dd);
            let err = format!("{err:?}");
            let line = to_wstring(&err);
            TextOutW(hdc, 32, start_y, line.as_ptr(), line.len() as _);
        }
    }

   // Define the rectangle's coordinates
   let left = 50; let top = 50; let right = 200; let bottom = 150;
   // Define the width and height of the ellipse for rounded corners
   let ellipseWidth = 30; let ellipseHeight = 30;
   // Draw a rounded rectangle
   RoundRect(hdc, left, top, right, bottom, ellipseWidth, ellipseHeight);
    
    EndPaint(hWnd, &ps) as LRESULT
}