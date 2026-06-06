use std::ptr::null_mut;
use ffi;
use winapi::{
    // ctypes::c_int,
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM},
        windef::{HDC, HWND, RECT},
    },
    um::{
        libloaderapi::GetModuleHandleW,
        wingdi::{CreateFontW, LOGFONTW},
        winuser::{
            BeginPaint, CreateWindowExW, DefWindowProcW, EndPaint, GetMessageW, PAINTSTRUCT,
            PostQuitMessage, RegisterClassW, ShowWindow, TranslateMessage, UpdateWindow,
            WM_DESTROY, WM_PAINT, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

const WINDOW_CLASS_NAME: &'static str = "my_window_class";

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        WM_PAINT => {
            let mut ps: PAINTSTRUCT = std::mem::zeroed();
            let hdc: HDC = BeginPaint(hwnd, &mut ps);

            let text = "Hello, world!";
            let x = 50;
            let y = 50;

            let logfont = LOGFONTW {
                lfHeight: -16,
                lfWidth: 0,
                lfEscapement: 0,
                lfOrientation: 0,
                lfWeight: 400,
                lfItalic: 0,
                lfUnderline: 0,
                lfStrikeOut: 0,
                lfCharSet: 0,
                lfOutPrecision: 0,
                lfClipPrecision: 0,
                lfQuality: 0,
                lfPitchAndFamily: 0,
                lfFaceName: [0; 32],
            };
            let hfont = CreateFontW(&logfont);
            let prev_font = winapi::um::wingdi::SelectObject(hdc, hfont as *mut _);

            winapi::um::wingdi::TextOutW(
                hdc,
                x,
                y,
                text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr(),
                text.len() as i32,
            );

            winapi::um::wingdi::SelectObject(hdc, prev_font);
            winapi::um::wingdi::DeleteObject(hfont as *mut _);

            EndPaint(hwnd, &ps);
            0
        }
        _ => DefWindowProcW(hwnd, msg, w_param, l_param),
    }
}

fn main() {
    unsafe {
        let hinstance: HINSTANCE = GetModuleHandleW(null_mut());
        let window_class = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: hinstance,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            lpszClassName: WINDOW_CLASS_NAME
                .encode_utf16()
                .chain(Some(0))
                .collect::<Vec<u16>>()
                .as_ptr(),
        };
        RegisterClassW(&window_class);

        let hwnd = CreateWindowExW(
            0,
            WINDOW_CLASS_NAME
                .encode_utf16()
                .chain(Some(0))
                .collect::<Vec<u16>>()
                .as_ptr(),
            "My Window".encode_utf16().collect::<Vec<u16>>().as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            100,
            100,
            640,
            480,
            null_mut(),
            null_mut(),
            hinstance,
            null_mut(),
        );
        if hwnd.is_null() {
            panic!("CreateWindowEx failed");
        }

        loop {
            let mut msg: MSG = std::mem::zeroed();
            if GetMessageW(&mut msg, hwnd, 0, 0) == 0 {
                break;
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
