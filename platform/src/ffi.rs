use core::ffi::c_void;

type StateHandle = u16;

static mut TEST_VALUE: i8 = -50;
static mut TEST_STRING: String = String::new();
static mut TEST_STRING_IS_READY: bool = false; 

#[no_mangle]
pub extern "C" fn tp_state_write(state_handle: StateHandle, data_type: u8, length: *const u16) -> *const c_void {
    unsafe {
        let ptr: *mut i8 = &mut TEST_VALUE;
        ptr as *const c_void
    }
}

#[no_mangle]
pub extern "C" fn tp_state_read(state_handle: StateHandle) -> i8 {
    unsafe {
        TEST_VALUE
    }
}

#[no_mangle]
pub extern "C" fn tp_init_str(len: u16) -> *const u8
{
    unsafe {
        if !TEST_STRING_IS_READY {
            TEST_STRING = String::with_capacity(len as usize);
            TEST_STRING_IS_READY = true;
        }
    }

    let timer = timer::Timer::new();
    let _guard = timer.schedule_with_delay(chrono::Duration::seconds(3), move || {
        unsafe {
            TEST_STRING.push('.');
        }
    });

    unsafe {
        TEST_STRING.as_ptr()
    }
}

// name mangling is necessary to create variants of `tp_state_write_generic` symbol // #[no_mangle]
pub extern "C" fn tp_state_write_generic<T>(state_handle: StateHandle, value: T) -> bool {
    true
}

#[no_mangle]
pub extern "C" fn tp_state_write_bool(state_handle: StateHandle, value: bool) -> bool {
    tp_state_write_generic(state_handle, value)
}

#[no_mangle]
pub extern "C" fn tp_state_write_string(state_handle: StateHandle, value: *const c_void) -> bool {
    tp_state_write_generic(state_handle, value)
}