extern crate winapi;

use std::ffi::CString;
use std::mem::size_of;
use winapi::shared::ntdef::HANDLE;
use winapi::shared::minwindef::{HINSTANCE, UINT, DWORD, WPARAM, LPARAM, LRESULT, LPVOID};
use winapi::shared::windef::{HWND, HMENU, HBRUSH, POINT};
use winapi::um::winnt::LPCSTR;
use winapi::um::winuser;
use winapi::shared::guiddef::GUID;
use winapi::shared::usbiodef;
use winapi::um::errhandlingapi as winerr;

#[repr(C)]
struct DEV_BROADCAST_DEVICEINTERFACE {
    dbcc_size: DWORD,
    dbcc_devicetype: DWORD,
    dbcc_reserved: DWORD,
    dbcc_classguid: GUID,
    dbcc_name: [u8; 1],
}

pub unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    println!("{:?}", msg);
    if msg == winuser::WM_DEVICECHANGE {
        println!("==========");
    } else if msg == winuser::WM_DESTROY {
        winuser::PostQuitMessage(0);
    }
    return winuser::DefWindowProcW(h_wnd, msg, w_param, l_param);
}

fn main() {
    unsafe {
        let class_name = CString::new("my_window").unwrap();
        let window_name = CString::new("my_window").unwrap();

        let wnd = winuser::WNDCLASSA {
            style: 0,
            lpfnWndProc: Some(window_proc), 
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: winuser::LoadIconW(0 as HINSTANCE, winuser::IDI_APPLICATION),
            hCursor: winuser::LoadCursorW(0 as HINSTANCE, winuser::IDI_APPLICATION),
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: 0 as LPCSTR,
            lpszClassName: class_name.as_ptr() as LPCSTR,
        };

        winuser::RegisterClassA(&wnd);

        let h_wnd_window = winuser::CreateWindowExA(
            0,
            class_name.as_ptr() as LPCSTR,
            window_name.as_ptr() as LPCSTR,
            // winuser::WS_OVERLAPPEDWINDOW | winuser::WS_VISIBLE,
            winuser::WS_OVERLAPPEDWINDOW,
            0,
            0,
            100,
            100,
            0 as HWND,
            0 as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut()
        );

        let filter = Box::new(DEV_BROADCAST_DEVICEINTERFACE{
            dbcc_size: size_of::<DEV_BROADCAST_DEVICEINTERFACE>() as u32,
            dbcc_devicetype: 0x00000005,
            dbcc_reserved: 0,
            dbcc_classguid: usbiodef::GUID_DEVINTERFACE_USB_DEVICE,
            dbcc_name: [0],
        });
        winuser::RegisterDeviceNotificationA(
            h_wnd_window as HANDLE,
            Box::into_raw(filter) as LPVOID,
            winuser::DEVICE_NOTIFY_WINDOW_HANDLE
        );

        // winuser::ShowWindow(h_wnd_window, winuser::SW_SHOW);

        let mut msg = winuser::MSG {
            hwnd : 0 as HWND,
            message : 0 as UINT,
            wParam : 0 as WPARAM,
            lParam : 0 as LPARAM,
            time : 0 as DWORD,
            pt : POINT { x: 0, y: 0, },
        };

        loop {   
            let pm = winuser::GetMessageW(&mut msg, 0 as HWND, 0, 0);
            if pm == 0 {
                break;
            }

            if msg.message == winuser::WM_QUIT {
                break;
            }

            winuser::TranslateMessage(&mut msg);
            winuser::DispatchMessageW(&mut msg);
        }
    }
}
