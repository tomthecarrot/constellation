use core::ffi::c_void;

type StateHandle = u16;
type DataType = u8;

static mut TEST_VALUE: i8 = -50;
static mut TEST_STRING: String = String::new();
static mut TEST_STRING_IS_READY: bool = false; 

#[no_mangle]
pub extern "C" fn tp_state_get_ptr(state_handle: StateHandle, data_type: u8, result: *mut u8) -> ResultCode {
    let mut result_code: ResultCode = RESULT_FAIL_UNKNOWN;

    unsafe {
        let data_value = &STORE.test_set.int8; // TEMP
        let data_ptr = data_value as *const i8;
        let generic_ptr = data_ptr as *const u8;

        *result = *generic_ptr; // MARK[UNSAFE_NEEDED]
        result_code = RESULT_OK; // MARK[UNSAFE_NEEDED]
    }
    
    result_code
}