use windows::{core::*, Win32::Foundation::*, Win32::{System::LibraryLoader::GetModuleHandleA, Graphics::Gdi::{ BeginPaint, PAINTSTRUCT, EndPaint}}, Win32::UI::WindowsAndMessaging::*};

pub fn win_main() -> Result<()> {
        
        let mut window = Window::new()?;
        window.run()
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
pub unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
pub unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
pub unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
pub unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}

pub struct Window {
    pub handle: HWND,
    pub dpi: f32,
    pub visible: bool,
    pub occlusion: u32,
    pub frequency: i64
}

impl Window {
    fn new() -> Result<Self> {
        let dpi = 0.0;
        let frequency = 0;

        Ok(Window {
            handle: HWND(0),
            dpi,
            visible: false,
            occlusion: 0,
            frequency
        })
    }


    fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match message {
                WM_PAINT => {
                    println!("WM_PAINT");
                    let mut ps = PAINTSTRUCT::default();
                    BeginPaint(self.handle, &mut ps);
                    EndPaint(self.handle, &ps);
                    LRESULT(0)
                }
                WM_SIZE => {
                    println!("WM_SIZE");
                    if wparam.0 != SIZE_MINIMIZED as usize {
                    }
                    LRESULT(0)
                }
                WM_DISPLAYCHANGE => {
                    println!("WM_DISPLAYCHANGE");
                    // self.render().unwrap();
                    LRESULT(0)
                }
                WM_USER => {
                    println!("WM_USER");
                    LRESULT(0)
                }
                WM_ACTIVATE => {
                    println!("ACTIVATE");
                    self.visible = true;
                    LRESULT(0)
                }
                WM_DESTROY => {
                    println!("WM_DESTROY");
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcA(self.handle, message, wparam, lparam),
            }
        }
    }

    fn run(&mut self) -> Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None);
            debug_assert!(instance.0 != 0);

            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_HAND),
                hInstance: instance,
                lpszClassName: PSTR(b"window\0".as_ptr()),

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wndproc),
                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(Default::default(), PSTR(b"window\0".as_ptr()), PSTR(b"ShaderDojo\0".as_ptr()), WS_OVERLAPPEDWINDOW | WS_VISIBLE, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, None, None, instance, self as *mut _ as _);

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
            let mut message = MSG::default();

            loop {
                if self.visible {

                    while PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
                        if message.message == WM_QUIT {
                            return Ok(());
                        }
                        DispatchMessageA(&message);
                    }
                } else {
                    GetMessageA(&mut message, None, 0, 0);

                    if message.message == WM_QUIT {
                        return Ok(());
                    }

                    DispatchMessageA(&message);
                }
            }
        }
    }

    extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            if message == WM_NCCREATE {
                let cs = lparam.0 as *const CREATESTRUCTA;
                let this = (*cs).lpCreateParams as *mut Self;
                (*this).handle = window;

                SetWindowLong(window, GWLP_USERDATA, this as _);
            } else {
                let this = GetWindowLong(window, GWLP_USERDATA) as *mut Self;

                if !this.is_null() {
                    return (*this).message_handler(message, wparam, lparam);
                }
            }

            DefWindowProcA(window, message, wparam, lparam)
        }
    }
}