#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type LPARAM = isize;
pub type LRESULT = isize;
pub type WPARAM = usize;
pub type BOOL = i32;

//type UINT=u16;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
pub type WNDCLASS_STYLES = u32;
pub type SYS_COLOR_INDEX = i32;
pub type COLORREF = u32;
pub type BACKGROUND_MODE = u32;
pub type DRAW_TEXT_FORMAT = u32;

pub type MUT_PTR_ANY = *mut core::ffi::c_void;

pub type HWND = MUT_PTR_ANY;

pub type HMODULE = MUT_PTR_ANY;
pub type HINSTANCE = MUT_PTR_ANY;
pub type HICON = MUT_PTR_ANY;
pub type HCURSOR = MUT_PTR_ANY;
pub type HBRUSH = MUT_PTR_ANY;
pub type HMENU = MUT_PTR_ANY;
pub type HDC = MUT_PTR_ANY;
pub type HGDIOBJ = MUT_PTR_ANY;
pub type HBITMAP = MUT_PTR_ANY;

pub type SHOW_WINDOW_CMD = i32;
pub type WINDOW_STYLE = u32;
pub type WINDOW_EX_STYLE = u32;
pub type GET_STOCK_OBJECT_FLAGS = i32;

pub type MENU_ITEM_MASK = u32;
pub type MENU_ITEM_TYPE = u32;
pub type MENU_ITEM_STATE = u32;
pub type MENU_ITEM_FLAGS = u32;

pub const CS_OWNDC: WNDCLASS_STYLES = 32u32;
pub const CS_HREDRAW: WNDCLASS_STYLES = 2u32;
pub const CS_VREDRAW: WNDCLASS_STYLES = 1u32;

pub const IDC_ARROW: PCWSTR = 32512u16 as _;
pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = 13565952u32;
pub const WS_VISIBLE: WINDOW_STYLE = 268435456u32;
pub const CW_USEDEFAULT: i32 = -2147483648i32;

pub const SW_SHOW: SHOW_WINDOW_CMD = 5i32;

pub const COLOR_BACKGROUND: SYS_COLOR_INDEX = 1i32;

pub const WM_DESTROY: u32 = 2u32;
pub const WM_PAINT: u32 = 15u32;

pub const TRANSPARENT: BACKGROUND_MODE = 1u32;
pub const DT_SINGLELINE: DRAW_TEXT_FORMAT = 32u32;
pub const DT_NOCLIP: DRAW_TEXT_FORMAT = 256u32;

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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Default for RECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [u8; 32],
}

#[repr(C)]
//#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct MENUITEMINFOW {
    pub cbSize: u32,
    pub fMask: MENU_ITEM_MASK,
    pub fType: MENU_ITEM_TYPE,
    pub fState: MENU_ITEM_STATE,
    pub wID: u32,
    pub hSubMenu: HMENU,
    pub hbmpChecked: HBITMAP,
    pub hbmpUnchecked: HBITMAP,
    pub dwItemData: usize,
    pub dwTypeData: PWSTR,
    pub cch: u32,
    pub hbmpItem: HBITMAP,
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
windows_targets::link!("user32.dll" "system" fn BeginPaint(hwnd : HWND, lppaint : *mut PAINTSTRUCT) -> HDC);
windows_targets::link!("user32.dll" "system" fn GetClientRect(hwnd : HWND, lprect : *mut RECT) -> BOOL);
windows_targets::link!("gdi32.dll" "system" fn SetTextColor(hdc : HDC, color : COLORREF) -> COLORREF);
windows_targets::link!("gdi32.dll" "system" fn SetBkMode(hdc : HDC, mode : i32) -> i32);
windows_targets::link!("user32.dll" "system" fn DrawTextW(hdc : HDC, lpchtext : PCWSTR, cchtext : i32, lprc : *mut RECT, format : DRAW_TEXT_FORMAT) -> i32);
windows_targets::link!("user32.dll" "system" fn EndPaint(hwnd :  HWND, lppaint : *const PAINTSTRUCT) -> BOOL);
windows_targets::link!("gdi32.dll" "system" fn DeleteDC(hdc : HDC) -> BOOL);
windows_targets::link!("user32.dll" "system" fn GetWindowDC(hwnd : HWND) -> HDC);
windows_targets::link!("gdi32.dll" "system" fn GetStockObject(i : GET_STOCK_OBJECT_FLAGS) -> HGDIOBJ);
windows_targets::link!("gdi32.dll" "system" fn TextOutW(hdc : HDC, x : i32, y : i32, lpstring : PCWSTR, c : i32) -> BOOL);

windows_targets::link!("user32.dll" "system" fn CreateMenu() -> HMENU);
windows_targets::link!("user32.dll" "system" fn DestroyMenu(hmenu : HMENU) -> BOOL);
windows_targets::link!("user32.dll" "system" fn InsertMenuItemW(hmenu : HMENU, item : u32, fbyposition : BOOL, lpmi : *const MENUITEMINFOW) -> BOOL);
windows_targets::link!("user32.dll" "system" fn InsertMenuW(hmenu : HMENU, uposition : u32, uflags : MENU_ITEM_FLAGS, uidnewitem : usize,
  lpnewitem : PCWSTR) -> BOOL);
  

pub fn RGB(r: u8, g: u8, b: u8) -> u32 {
    r as u32 | (g as u32) << 8 | (b as u32) << 16
}

