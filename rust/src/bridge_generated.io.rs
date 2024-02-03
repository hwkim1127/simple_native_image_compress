use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_fit_width(
    port_: i64,
    path_str: *mut wire_uint_8_list,
    compress_format: *mut i32,
    max_width: *mut i32,
    quality: *mut u8,
    sampling_filter: *mut i32,
) {
    wire_fit_width_impl(
        port_,
        path_str,
        compress_format,
        max_width,
        quality,
        sampling_filter,
    )
}

#[no_mangle]
pub extern "C" fn wire_fit_height(
    port_: i64,
    path_str: *mut wire_uint_8_list,
    compress_format: *mut i32,
    max_height: *mut i32,
    quality: *mut u8,
    sampling_filter: *mut i32,
) {
    wire_fit_height_impl(
        port_,
        path_str,
        compress_format,
        max_height,
        quality,
        sampling_filter,
    )
}

#[no_mangle]
pub extern "C" fn wire_contain(
    port_: i64,
    path_str: *mut wire_uint_8_list,
    compress_format: *mut i32,
    max_width: *mut i32,
    max_height: *mut i32,
    quality: *mut u8,
    sampling_filter: *mut i32,
) {
    wire_contain_impl(
        port_,
        path_str,
        compress_format,
        max_width,
        max_height,
        quality,
        sampling_filter,
    )
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_compress_format_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_filter_type_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i32_0(value: i32) -> *mut i32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<CompressFormat> for *mut i32 {
    fn wire2api(self) -> CompressFormat {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<CompressFormat>::wire2api(*wrap).into()
    }
}
impl Wire2Api<FilterType> for *mut i32 {
    fn wire2api(self) -> FilterType {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<FilterType>::wire2api(*wrap).into()
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
