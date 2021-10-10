use core::ffi::c_void;

type ObjectHandle = u16;
type StateHandle = u16;
type DataType = u8;

static mut TEST_VALUE: i8 = -50;

#[no_mangle]
pub extern "C" fn tp_property_get_ptr(object_handle: ObjectHandle, property_handle: StateHandle, data_type: u8, result_ptr: *mut u8) -> TPResultCode {
    let mut result_code: TPResultCode = TPResultCode::RESULT_FAIL_UNKNOWN;

    unsafe {
        let data_value = TEST_VALUE; // TEMP
        let data_ptr = data_value as *const i8; // TEMP
        let generic_ptr = data_ptr as *const u8; // TEMP

        *result_ptr = *generic_ptr; // MARK[UNSAFE_NEEDED]
        result_code = RESULT_OK; // MARK[UNSAFE_NEEDED]
    }
    
    result_code
}

#[no_mangle]
pub extern "C" fn tp_property_flag() {
    // TODO
}

#[no_mangle]
pub extern "C" fn tp_property_arm(object_handle: ObjectHandle, property_handle: StateHandle, armed: bool) -> TPResultCode {
    // TODO
}