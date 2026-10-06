use omarchy_app_starter::Counter;
use std::ffi::{c_char, c_void, CString};

extern "C" {
    fn run_window(
        app_id: *const c_char,
        title: *const c_char,
        state: *mut c_void,
        increment: extern "C" fn(*mut c_void) -> u32,
        smoke_test: bool,
    ) -> i32;
}

extern "C" fn increment(state: *mut c_void) -> u32 {
    // SAFETY: run_window invokes callbacks synchronously on its UI thread and
    // does not retain this pointer after the event loop returns.
    unsafe { (*state.cast::<Counter>()).increment() }
}

fn main() {
    let app_id = CString::new("io.github.tcballard.app_starter").expect("static application ID");
    let title = CString::new("Omarchy App Starter").expect("static title");
    let smoke_test = std::env::args().any(|a| a == "--smoke-test");
    let mut state = Counter::default();
    // SAFETY: both C strings and state remain alive for the entire event loop.
    let code = unsafe {
        run_window(
            app_id.as_ptr(),
            title.as_ptr(),
            (&mut state as *mut Counter).cast(),
            increment,
            smoke_test,
        )
    };
    std::process::exit(code);
}
