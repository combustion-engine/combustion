#![allow(unused_imports, bad_style)]

extern crate glfw;
extern crate nice_glfw;

extern crate winapi;
extern crate kernel32;
extern crate user32;
extern crate gdi32;
extern crate d3d11;
extern crate dxgi;

#[macro_use]
extern crate combustion_common as common;

use std::mem::{size_of, zeroed, transmute, transmute_copy};
use std::ptr;

use common::error::*;

use glfw::{Window, WindowHint, WindowEvent, Action, Key};

use kernel32::{GetModuleHandleA};
use user32::{RegisterClassExW, CreateWindowExW, ShowWindow, MessageBoxA};
use user32::{GetMessageW, TranslateMessage, DispatchMessageW};
use user32::{DefWindowProcW, PostQuitMessage, BeginPaint};
use gdi32::{TextOutA};

use winapi::*;
use d3d11::*;
use dxgi::*;

struct D3DState {
    hWnd: HWND,
    swapchain: *mut IDXGISwapChain,
    dev: *mut ID3D11Device,
    devcon: *mut ID3D11DeviceContext,
}

unsafe fn InitD3D(mut state: &mut D3DState) {
    let mut scd: DXGI_SWAP_CHAIN_DESC = zeroed();

    scd.BufferCount = 1;                                    // one back buffer
    scd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;     // use 32-bit color
    scd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;      // how swap chain is to be used
    scd.OutputWindow = state.hWnd;                          // the window to be used
    scd.SampleDesc.Count = 4;                               // how many multisamples
    scd.Windowed = TRUE;                                    // windowed/full-screen mode

    // create a device, device context and swap chain using the information in the scd struct
    let res = D3D11CreateDeviceAndSwapChain(ptr::null_mut(),
                                            D3D_DRIVER_TYPE_HARDWARE,
                                            ptr::null_mut(),
                                            D3D11_CREATE_DEVICE_DEBUG.0,
                                            ptr::null(),
                                            0,
                                            D3D11_SDK_VERSION,
                                            &scd,
                                            &mut state.swapchain,
                                            &mut state.dev,
                                            ptr::null_mut(),
                                            &mut state.devcon);

    println!("Here {}", res);
}

unsafe fn CleanD3D(state: &D3DState) {
    (*state.swapchain).Release();
    (*state.dev).Release();
    (*state.devcon).Release();
}

fn main() {
    common::log::init_global_logger("logs").expect("Could not initialize logging system!");

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).expect_logged_box("Could not initialize GLFW!");

    let (mut window, events) = nice_glfw::WindowBuilder::new(&mut glfw)
        .try_modern_context_hints()
        .size(1280, 720)
        //.aspect_ratio(16, 9)
        .common_hints(&[
            WindowHint::Visible(true),
            //WindowHint::Samples(Some(2)),
            WindowHint::DoubleBuffer(true),
            //WindowHint::OpenGlDebugContext(true),
        ])
        .title("Combustion")
        .create()
        .expect_logged_box("Couldn't create window");

    info!("Window created");

    //Enable interactivity
    window.set_all_polling(true);

    let mut d3dstate = unsafe {
        D3DState {
            hWnd: transmute(window.get_win32_window()),
            swapchain: ptr::null_mut(),
            dev: ptr::null_mut(),
            devcon: ptr::null_mut()
        }
    };

    unsafe {
        InitD3D(&mut d3dstate);
    }

    while !window.should_close() {
        glfw.wait_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }
    }

    unsafe {
        CleanD3D(&d3dstate);
    }
}