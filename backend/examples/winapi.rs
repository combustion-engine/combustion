extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;

use std::mem::{size_of, zeroed};

use winapi::{UINT, WPARAM, LPARAM, LRESULT, LPVOID, LPCSTR, LPCWSTR};
use winapi::{HWND, HMENU, HICON, HCURSOR, HBRUSH};
use winapi::{WNDCLASSEXW, CS_VREDRAW, CS_HREDRAW, COLOR_WINDOWFRAME, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT};
use winapi::{SW_SHOWDEFAULT, WM_DESTROY, WM_PAINT};
use kernel32::{GetModuleHandleA};
use user32::{RegisterClassExW, CreateWindowExW, ShowWindow, MessageBoxA};
use user32::{GetMessageW, TranslateMessage, DispatchMessageW};
use user32::{DefWindowProcW, PostQuitMessage, BeginPaint};
use gdi32::{TextOutA};

use kernel32::{FreeConsole};

static SZ_CLASS: &'static [u8] = b"c\0l\0a\0s\0s\0\0\0";
static SZ_TITLE: &'static [u8] = b"t\0i\0t\0l\0e\0\0\0";
static SZ_TEXT: &'static [u8] = b"Hello, world!";

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        },
        WM_PAINT => {
            let mut ps = zeroed();
            let hdc = BeginPaint(hwnd, &mut ps);
            TextOutA(hdc, 5, 5,
                     SZ_TEXT.as_ptr() as *const i8,
                     SZ_TEXT.len() as i32
            );
            0
        },
        _ => {
            DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

fn main() {
    unsafe {
        let h_instance = GetModuleHandleA(0 as LPCSTR);
        let wcex = WNDCLASSEXW {
            cbSize: size_of::<WNDCLASSEXW>() as u32,
            style: CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: 0 as HICON,
            hCursor: 0 as HCURSOR,
            hbrBackground: (COLOR_WINDOWFRAME) as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: SZ_CLASS.as_ptr() as *const u16,
            hIconSm: 0 as HICON,
        };
        match RegisterClassExW(&wcex) {
            0 => {
                MessageBoxA(
                    0 as HWND,
                    b"Call to RegisterClassEx failed!\0".as_ptr() as *const i8,
                    b"Win32 Guided Tour\0".as_ptr() as *const i8,
                    0 as UINT
                );
            },
            _atom => {
                let window = CreateWindowExW(
                    0,
                    SZ_CLASS.as_ptr() as *const u16,
                    SZ_TITLE.as_ptr() as *const u16,
                    WS_OVERLAPPEDWINDOW,
                    CW_USEDEFAULT, CW_USEDEFAULT, 500, 100,
                    0 as HWND, 0 as HMENU,
                    h_instance,
                    0 as LPVOID
                );
                if window.is_null() {
                    MessageBoxA(
                        0 as HWND,
                        b"Call to CreateWindow failed!\0".as_ptr() as *const i8,
                        b"Win32 Guided Tour\0".as_ptr() as *const i8,
                        0 as UINT
                    );
                } else {
                    ShowWindow(window, SW_SHOWDEFAULT);
                    FreeConsole();
                    let mut msg = zeroed();
                    while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                };
            }
        };
    }
}