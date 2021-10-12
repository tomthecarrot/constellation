#![macro_use]
use lazy_mut::lazy_mut;

use core::ffi::c_void;

use crate::TPResultCode;
use crate::TestSet;

type ObjectHandle = u16;
type StateHandle = u16;
type DataType = u8;

// lazy_mut! {
//     static mut set: TestSet = TestSet::default();
// }

#[no_mangle]
pub extern "C" fn tp_property_get_ptr(object_handle: ObjectHandle, property_handle: StateHandle, data_type: DataType, result_ptr: *mut u8) -> TPResultCode {
    let mut result_code: TPResultCode = TPResultCode::FailUnknown;

    unsafe {
        let data_value: i8 = -69; //set.int8; // TEMP
        let data_ptr = &data_value as *const i8; // TEMP
        let generic_ptr = data_ptr as *const u8; // TEMP

        *result_ptr = *generic_ptr; // MARK[UNSAFE_NEEDED]
        result_code = TPResultCode::Ok; // MARK[UNSAFE_NEEDED]
    }
    
    result_code
}

#[no_mangle]
pub extern "C" fn tp_property_flag() {
    todo!()
}

#[no_mangle]
pub extern "C" fn tp_property_arm(object_handle: ObjectHandle, property_handle: StateHandle, armed: bool) -> TPResultCode {
    todo!()
}