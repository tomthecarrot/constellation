#![macro_use]
use lazy_mut::{ lazy_mut, LazyMut };

use crate::TPResultCode;
use crate::TestSet;

type ObjectHandle = u16;
type StateHandle = u16;
type DataType = u8;

lazy_mut! {
    static mut TEST_SET: TestSet = TestSet::default();
}

#[no_mangle]
pub extern "C" fn tp_init() -> TPResultCode {
    unsafe {
        TEST_SET = LazyMut::Value(TestSet::default());
    }
    
    TPResultCode::Ok
}

#[no_mangle]
pub extern "C" fn tp_property_get_ptr(object_handle: ObjectHandle, property_handle: StateHandle, data_type: DataType, data_addr: *mut usize) -> TPResultCode {
    let mut result_code: TPResultCode = TPResultCode::FailUnknown;

    unsafe {
        let data_value_ref = &TEST_SET.int8;
        let data_ptr_raw = data_value_ref as *const i8;
        *data_addr = data_ptr_raw as usize;

        result_code = TPResultCode::Ok;
    }
    
    result_code
}

#[no_mangle]
pub extern "C" fn tp_property_flag() -> TPResultCode {
    // TODO
    TPResultCode::NotImplemented
}

#[no_mangle]
pub extern "C" fn tp_property_arm(object_handle: ObjectHandle, property_handle: StateHandle, armed: bool) -> TPResultCode {
    // TODO
    TPResultCode::NotImplemented
}