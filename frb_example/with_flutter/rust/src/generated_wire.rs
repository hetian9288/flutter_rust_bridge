#![allow(
    non_camel_case_types,
    clippy::redundant_closure,
    clippy::useless_conversion
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
use flutter_rust_bridge::*;

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_draw_mandelbrot(
    port: i64,
    image_size: *mut wire_Size,
    zoom_point: *mut wire_Point,
    scale: f64,
    num_threads: i32,
) {
    let api_image_size = image_size.wire2api();
    let api_zoom_point = zoom_point.wire2api();
    let api_scale = scale.wire2api();
    let api_num_threads = num_threads.wire2api();
    support::wrap_wire_func(port, move || {
        draw_mandelbrot(api_image_size, api_zoom_point, api_scale, api_num_threads)
    });
}

#[no_mangle]
pub extern "C" fn wire_passing_complex_structs(port: i64, root: *mut wire_TreeNode) {
    let api_root = root.wire2api();
    support::wrap_wire_func(port, move || passing_complex_structs(api_root));
}

#[no_mangle]
pub extern "C" fn wire_memory_test_utility_input_array(port: i64, input: *mut wire_uint_8_list) {
    let api_input = input.wire2api();
    support::wrap_wire_func(port, move || memory_test_utility_input_array(api_input));
}

#[no_mangle]
pub extern "C" fn wire_memory_test_utility_output_zero_copy_buffer(port: i64, len: i32) {
    let api_len = len.wire2api();
    support::wrap_wire_func(port, move || {
        memory_test_utility_output_zero_copy_buffer(api_len)
    });
}

#[no_mangle]
pub extern "C" fn wire_memory_test_utility_output_vec_u8(port: i64, len: i32) {
    let api_len = len.wire2api();
    support::wrap_wire_func(port, move || memory_test_utility_output_vec_u8(api_len));
}

#[no_mangle]
pub extern "C" fn wire_memory_test_utility_input_vec_size(port: i64, input: *mut wire_list_size) {
    let api_input = input.wire2api();
    support::wrap_wire_func(port, move || memory_test_utility_input_vec_size(api_input));
}

#[no_mangle]
pub extern "C" fn wire_memory_test_utility_output_vec_size(port: i64, len: i32) {
    let api_len = len.wire2api();
    support::wrap_wire_func(port, move || memory_test_utility_output_vec_size(api_len));
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_Size {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Point {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_TreeNode {
    name: *mut wire_uint_8_list,
    children: *mut wire_list_tree_node,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_tree_node {
    ptr: *mut wire_TreeNode,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_size {
    ptr: *mut wire_Size,
    len: i32,
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_size() -> *mut wire_Size {
    support::new_leak_box_ptr(wire_Size::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_point() -> *mut wire_Point {
    support::new_leak_box_ptr(wire_Point::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tree_node() -> *mut wire_TreeNode {
    support::new_leak_box_ptr(wire_TreeNode::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_tree_node(len: i32) -> *mut wire_list_tree_node {
    let wrap = wire_list_tree_node {
        ptr: support::new_leak_vec_ptr(wire_TreeNode::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_size(len: i32) -> *mut wire_list_size {
    let wrap = wire_list_size {
        ptr: support::new_leak_vec_ptr(wire_Size::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl Wire2Api<Size> for *mut wire_Size {
    fn wire2api(self) -> Size {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Size> for wire_Size {
    fn wire2api(self) -> Size {
        Size {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}

impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<Point> for *mut wire_Point {
    fn wire2api(self) -> Point {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Point> for wire_Point {
    fn wire2api(self) -> Point {
        Point {
            x: self.x.wire2api(),
            y: self.y.wire2api(),
        }
    }
}

impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}

impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
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

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<TreeNode> for *mut wire_TreeNode {
    fn wire2api(self) -> TreeNode {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<TreeNode> for wire_TreeNode {
    fn wire2api(self) -> TreeNode {
        TreeNode {
            name: self.name.wire2api(),
            children: self.children.wire2api(),
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<TreeNode>> for *mut wire_list_tree_node {
    fn wire2api(self) -> Vec<TreeNode> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(|x| x.wire2api()).collect()
    }
}

impl Wire2Api<Vec<Size>> for *mut wire_list_size {
    fn wire2api(self) -> Vec<Size> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(|x| x.wire2api()).collect()
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl NewWithNullPtr for wire_Size {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_Point {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl NewWithNullPtr for wire_TreeNode {
    fn new_with_null_ptr() -> Self {
        Self {
            name: std::ptr::null_mut(),
            children: std::ptr::null_mut(),
        }
    }
}

// Section: impl IntoDart

impl support::IntoDart for Size {
    fn into_dart(self) -> support::DartCObject {
        vec![self.width.into_dart(), self.height.into_dart()].into_dart()
    }
}

impl support::IntoDart for Point {
    fn into_dart(self) -> support::DartCObject {
        vec![self.x.into_dart(), self.y.into_dart()].into_dart()
    }
}

impl support::IntoDart for TreeNode {
    fn into_dart(self) -> support::DartCObject {
        vec![self.name.into_dart(), self.children.into_dart()].into_dart()
    }
}