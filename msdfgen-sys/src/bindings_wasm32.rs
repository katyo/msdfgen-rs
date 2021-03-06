/* automatically generated by rust-bindgen */

pub const MSDFGEN_CUBIC_SEARCH_STARTS: u32 = 4;
pub const MSDFGEN_CUBIC_SEARCH_STEPS: u32 = 4;
pub const MSDFGEN_VERSION: &'static [u8; 4usize] = b"1.6\0";
pub type std_size_t = ::std::os::raw::c_ulong;
pub type std_integral_constant_value_type<_Tp> = _Tp;
pub type std_integral_constant_type = u8;
pub type std_true_type = u8;
pub type std_false_type = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___and_ {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_is_empty {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_make_unsigned {
    pub _address: u8,
}
pub type std_make_unsigned_type = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub union std_aligned_storage_type {
    pub __data: *mut ::std::os::raw::c_uchar,
    pub __align: std_aligned_storage_type__bindgen_ty_1,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_aligned_storage_type__bindgen_ty_1 {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_std_aligned_storage_type() {
    assert_eq!(
        ::std::mem::size_of::<std_aligned_storage_type>(),
        4usize,
        concat!("Size of: ", stringify!(std_aligned_storage_type))
    );
    assert_eq!(
        ::std::mem::align_of::<std_aligned_storage_type>(),
        4usize,
        concat!("Alignment of ", stringify!(std_aligned_storage_type))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___detector {
    pub _address: u8,
}
pub type std___detector_value_t = std_false_type;
pub type std___detector_type<_Default> = _Default;
pub type std___detected_or = std___detector;
pub type std___detected_or_t = std___detected_or;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_iterator {
    pub _address: u8,
}
pub type std_iterator_iterator_category<_Category> = _Category;
pub type std_iterator_value_type<_Tp> = _Tp;
pub type std_iterator_difference_type<_Distance> = _Distance;
pub type std_iterator_pointer<_Pointer> = _Pointer;
pub type std_iterator_reference<_Reference> = _Reference;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___iterator_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_iterator_traits {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___undefined {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___get_first_arg {
    pub _address: u8,
}
pub type std___get_first_arg_type = std___undefined;
pub type std___get_first_arg_t = std___get_first_arg;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___replace_first_arg {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_pointer_traits {
    pub _address: u8,
}
pub type std_pointer_traits___element_type = [u8; 0usize];
pub type std_pointer_traits___difference_type = [u8; 0usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_pointer_traits___rebind {
    pub _address: u8,
}
pub type std_pointer_traits_pointer<_Ptr> = _Ptr;
pub type std_pointer_traits_element_type = std___detected_or_t;
pub type std_pointer_traits_difference_type = std___detected_or_t;
pub type std_pointer_traits_rebind = std_pointer_traits___rebind;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_reverse_iterator<_Iterator> {
    pub current: _Iterator,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
}
pub type std_reverse_iterator___traits_type = std_iterator_traits;
pub type std_reverse_iterator_iterator_type<_Iterator> = _Iterator;
pub type std_reverse_iterator_difference_type = std_reverse_iterator___traits_type;
pub type std_reverse_iterator_pointer = std_reverse_iterator___traits_type;
pub type std_reverse_iterator_reference = std_reverse_iterator___traits_type;
pub type std___allocator_base = __gnu_cxx_new_allocator;
#[repr(C)]
#[derive(Debug)]
pub struct std_allocator {
    pub _address: u8,
}
pub type std_allocator_size_type = std_size_t;
pub type std_allocator_difference_type = isize;
pub type std_allocator_pointer<_Tp> = *mut _Tp;
pub type std_allocator_const_pointer<_Tp> = *const _Tp;
pub type std_allocator_reference<_Tp> = *mut _Tp;
pub type std_allocator_const_reference<_Tp> = *const _Tp;
pub type std_allocator_value_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_rebind {
    pub _address: u8,
}
pub type std_allocator_rebind_other = std_allocator;
pub type std_allocator_propagate_on_container_move_assignment = std_true_type;
pub type std_allocator_is_always_equal = std_true_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___allocator_traits_base {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std___allocator_traits_base___rebind {
    pub _address: u8,
}
pub type std___allocator_traits_base___pointer = [u8; 0usize];
pub type std___allocator_traits_base___c_pointer = [u8; 0usize];
pub type std___allocator_traits_base___v_pointer = [u8; 0usize];
pub type std___allocator_traits_base___cv_pointer = [u8; 0usize];
pub type std___allocator_traits_base___pocca = [u8; 0usize];
pub type std___allocator_traits_base___pocma = [u8; 0usize];
pub type std___allocator_traits_base___pocs = [u8; 0usize];
pub type std___allocator_traits_base___equal = [u8; 0usize];
#[test]
fn bindgen_test_layout_std___allocator_traits_base() {
    assert_eq!(
        ::std::mem::size_of::<std___allocator_traits_base>(),
        1usize,
        concat!("Size of: ", stringify!(std___allocator_traits_base))
    );
    assert_eq!(
        ::std::mem::align_of::<std___allocator_traits_base>(),
        1usize,
        concat!("Alignment of ", stringify!(std___allocator_traits_base))
    );
}
pub type std___alloc_rebind = std___allocator_traits_base;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits {
    pub _address: u8,
}
pub type std_allocator_traits_allocator_type<_Alloc> = _Alloc;
pub type std_allocator_traits_value_type = [u8; 0usize];
pub type std_allocator_traits_pointer = std___detected_or_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits__Ptr {
    pub _address: u8,
}
pub type std_allocator_traits__Ptr_type = [u8; 0usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits__Diff {
    pub _address: u8,
}
pub type std_allocator_traits__Diff_type = std_pointer_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits__Size {
    pub _address: u8,
}
pub type std_allocator_traits_const_pointer = [u8; 0usize];
pub type std_allocator_traits_void_pointer = std_allocator_traits__Ptr;
pub type std_allocator_traits_const_void_pointer = std_allocator_traits__Ptr;
pub type std_allocator_traits_difference_type = [u8; 0usize];
pub type std_allocator_traits_size_type = [u8; 0usize];
pub type std_allocator_traits_propagate_on_container_copy_assignment = std___detected_or_t;
pub type std_allocator_traits_propagate_on_container_move_assignment = std___detected_or_t;
pub type std_allocator_traits_propagate_on_container_swap = std___detected_or_t;
pub type std_allocator_traits_is_always_equal = std___detected_or_t;
pub type std_allocator_traits_rebind_alloc = std___alloc_rebind;
pub type std_allocator_traits_rebind_traits = std_allocator_traits;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_allocator_traits___construct_helper {
    pub _address: u8,
}
pub type std_allocator_traits___construct_helper_type<_Alloc> = _Alloc;
pub type std_allocator_traits___has_construct = std_allocator_traits___construct_helper;
#[repr(C)]
pub struct std__Vector_base {
    pub _M_impl: std__Vector_base__Vector_impl,
}
pub type std__Vector_base__Tp_alloc_type = [u8; 0usize];
pub type std__Vector_base_pointer = [u8; 0usize];
#[repr(C)]
pub struct std__Vector_base__Vector_impl {
    pub _M_start: std__Vector_base_pointer,
    pub _M_finish: std__Vector_base_pointer,
    pub _M_end_of_storage: std__Vector_base_pointer,
}
pub type std__Vector_base_allocator_type<_Alloc> = _Alloc;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct std_vector {
    pub _address: u8,
}
pub type std_vector__Base = std__Vector_base;
pub type std_vector__Tp_alloc_type = std_vector__Base;
pub type std_vector__Alloc_traits = __gnu_cxx___alloc_traits;
pub type std_vector_value_type<_Tp> = _Tp;
pub type std_vector_pointer = std_vector__Base;
pub type std_vector_const_pointer = std_vector__Alloc_traits;
pub type std_vector_reference = std_vector__Alloc_traits;
pub type std_vector_const_reference = std_vector__Alloc_traits;
pub type std_vector_iterator = __gnu_cxx___normal_iterator<std_vector_pointer>;
pub type std_vector_const_iterator = __gnu_cxx___normal_iterator<std_vector_const_pointer>;
pub type std_vector_const_reverse_iterator = std_reverse_iterator<std_vector_const_iterator>;
pub type std_vector_reverse_iterator = std_reverse_iterator<std_vector_iterator>;
pub type std_vector_size_type = std_size_t;
pub type std_vector_difference_type = isize;
pub type std_vector_allocator_type<_Alloc> = _Alloc;
#[repr(C)]
#[derive(Debug)]
pub struct std_vector__Temporary_value {
    pub _M_this: *mut u8,
    pub __buf: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___normal_iterator<_Iterator> {
    pub _M_current: _Iterator,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Iterator>>,
}
pub type __gnu_cxx___normal_iterator___traits_type = std_iterator_traits;
pub type __gnu_cxx___normal_iterator_iterator_type<_Iterator> = _Iterator;
pub type __gnu_cxx___normal_iterator_iterator_category = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_value_type = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_difference_type = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_reference = __gnu_cxx___normal_iterator___traits_type;
pub type __gnu_cxx___normal_iterator_pointer = __gnu_cxx___normal_iterator___traits_type;
#[repr(C)]
#[derive(Debug)]
pub struct __gnu_cxx_new_allocator {
    pub _address: u8,
}
pub type __gnu_cxx_new_allocator_size_type = std_size_t;
pub type __gnu_cxx_new_allocator_difference_type = isize;
pub type __gnu_cxx_new_allocator_pointer<_Tp> = *mut _Tp;
pub type __gnu_cxx_new_allocator_const_pointer<_Tp> = *const _Tp;
pub type __gnu_cxx_new_allocator_reference<_Tp> = *mut _Tp;
pub type __gnu_cxx_new_allocator_const_reference<_Tp> = *const _Tp;
pub type __gnu_cxx_new_allocator_value_type<_Tp> = _Tp;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx_new_allocator_rebind {
    pub _address: u8,
}
pub type __gnu_cxx_new_allocator_rebind_other = __gnu_cxx_new_allocator;
pub type __gnu_cxx_new_allocator_propagate_on_container_move_assignment = std_true_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___alloc_traits {
    pub _address: u8,
}
pub type __gnu_cxx___alloc_traits_allocator_type<_Alloc> = _Alloc;
pub type __gnu_cxx___alloc_traits__Base_type = std_allocator_traits;
pub type __gnu_cxx___alloc_traits_value_type = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_pointer = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_const_pointer = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_size_type = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_difference_type = __gnu_cxx___alloc_traits__Base_type;
pub type __gnu_cxx___alloc_traits_reference = *mut __gnu_cxx___alloc_traits_value_type;
pub type __gnu_cxx___alloc_traits_const_reference = *const __gnu_cxx___alloc_traits_value_type;
pub type __gnu_cxx___alloc_traits___is_custom_pointer = std___and_;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __gnu_cxx___alloc_traits_rebind {
    pub _address: u8,
}
pub type __gnu_cxx___alloc_traits_rebind_other = __gnu_cxx___alloc_traits__Base_type;
pub type size_t = ::std::os::raw::c_ulong;
pub type __quad_t = ::std::os::raw::c_longlong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = __quad_t;
#[doc = " A 2-dimensional euclidean vector with double precision."]
#[doc = " Implementation based on the Vector2 template from Artery Engine."]
#[doc = " @author Viktor Chlumsky"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msdfgen_Vector2 {
    pub x: f64,
    pub y: f64,
}
#[test]
fn bindgen_test_layout_msdfgen_Vector2() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_Vector2>(),
        16usize,
        concat!("Size of: ", stringify!(msdfgen_Vector2))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_Vector2>(),
        8usize,
        concat!("Alignment of ", stringify!(msdfgen_Vector2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Vector2>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Vector2),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Vector2>())).y as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Vector2),
            "::",
            stringify!(y)
        )
    );
}
#[doc = " A vector may also represent a point, which shall be differentiated semantically using the alias Point2."]
pub type msdfgen_Point2 = msdfgen_Vector2;
pub const msdfgen_FillRule_FILL_NONZERO: msdfgen_FillRule = 0;
pub const msdfgen_FillRule_FILL_ODD: msdfgen_FillRule = 1;
pub const msdfgen_FillRule_FILL_POSITIVE: msdfgen_FillRule = 2;
pub const msdfgen_FillRule_FILL_NEGATIVE: msdfgen_FillRule = 3;
#[doc = " Fill rule dictates how intersection total is interpreted during rasterization."]
pub type msdfgen_FillRule = u32;
#[doc = " Represents a horizontal scanline intersecting a shape."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_Scanline {
    pub intersections: [u32; 3usize],
    pub lastIndex: ::std::os::raw::c_int,
}
#[doc = " An intersection with the scanline."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msdfgen_Scanline_Intersection {
    #[doc = " X coordinate."]
    pub x: f64,
    #[doc = " Normalized Y direction of the oriented edge at the point of intersection."]
    pub direction: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_msdfgen_Scanline_Intersection() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_Scanline_Intersection>(),
        16usize,
        concat!("Size of: ", stringify!(msdfgen_Scanline_Intersection))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_Scanline_Intersection>(),
        8usize,
        concat!("Alignment of ", stringify!(msdfgen_Scanline_Intersection))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Scanline_Intersection>())).x as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Scanline_Intersection),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<msdfgen_Scanline_Intersection>())).direction as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Scanline_Intersection),
            "::",
            stringify!(direction)
        )
    );
}
#[test]
fn bindgen_test_layout_msdfgen_Scanline() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_Scanline>(),
        16usize,
        concat!("Size of: ", stringify!(msdfgen_Scanline))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_Scanline>(),
        4usize,
        concat!("Alignment of ", stringify!(msdfgen_Scanline))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Scanline>())).intersections as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Scanline),
            "::",
            stringify!(intersections)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Scanline>())).lastIndex as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Scanline),
            "::",
            stringify!(lastIndex)
        )
    );
}
#[doc = " Represents a signed distance and alignment, which together can be compared to uniquely determine the closest edge segment."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct msdfgen_SignedDistance {
    pub distance: f64,
    pub dot: f64,
}
#[test]
fn bindgen_test_layout_msdfgen_SignedDistance() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_SignedDistance>(),
        16usize,
        concat!("Size of: ", stringify!(msdfgen_SignedDistance))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_SignedDistance>(),
        8usize,
        concat!("Alignment of ", stringify!(msdfgen_SignedDistance))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_SignedDistance>())).distance as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_SignedDistance),
            "::",
            stringify!(distance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_SignedDistance>())).dot as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_SignedDistance),
            "::",
            stringify!(dot)
        )
    );
}
pub const msdfgen_EdgeColor_BLACK: msdfgen_EdgeColor = 0;
pub const msdfgen_EdgeColor_RED: msdfgen_EdgeColor = 1;
pub const msdfgen_EdgeColor_GREEN: msdfgen_EdgeColor = 2;
pub const msdfgen_EdgeColor_YELLOW: msdfgen_EdgeColor = 3;
pub const msdfgen_EdgeColor_BLUE: msdfgen_EdgeColor = 4;
pub const msdfgen_EdgeColor_MAGENTA: msdfgen_EdgeColor = 5;
pub const msdfgen_EdgeColor_CYAN: msdfgen_EdgeColor = 6;
pub const msdfgen_EdgeColor_WHITE: msdfgen_EdgeColor = 7;
#[doc = " Edge color specifies which color channels an edge belongs to."]
pub type msdfgen_EdgeColor = u32;
#[repr(C)]
pub struct msdfgen_EdgeSegment__bindgen_vtable(::std::os::raw::c_void);
#[doc = " An abstract edge segment."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_EdgeSegment {
    pub vtable_: *const msdfgen_EdgeSegment__bindgen_vtable,
    pub color: msdfgen_EdgeColor,
}
#[test]
fn bindgen_test_layout_msdfgen_EdgeSegment() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_EdgeSegment>(),
        8usize,
        concat!("Size of: ", stringify!(msdfgen_EdgeSegment))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_EdgeSegment>(),
        4usize,
        concat!("Alignment of ", stringify!(msdfgen_EdgeSegment))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_EdgeSegment>())).color as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_EdgeSegment),
            "::",
            stringify!(color)
        )
    );
}
#[doc = " A line segment."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_LinearSegment {
    pub _base: msdfgen_EdgeSegment,
    pub p: [msdfgen_Point2; 2usize],
}
#[test]
fn bindgen_test_layout_msdfgen_LinearSegment() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_LinearSegment>(),
        40usize,
        concat!("Size of: ", stringify!(msdfgen_LinearSegment))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_LinearSegment>(),
        8usize,
        concat!("Alignment of ", stringify!(msdfgen_LinearSegment))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_LinearSegment>())).p as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_LinearSegment),
            "::",
            stringify!(p)
        )
    );
}
#[doc = " A quadratic Bezier curve."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_QuadraticSegment {
    pub _base: msdfgen_EdgeSegment,
    pub p: [msdfgen_Point2; 3usize],
}
#[test]
fn bindgen_test_layout_msdfgen_QuadraticSegment() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_QuadraticSegment>(),
        56usize,
        concat!("Size of: ", stringify!(msdfgen_QuadraticSegment))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_QuadraticSegment>(),
        8usize,
        concat!("Alignment of ", stringify!(msdfgen_QuadraticSegment))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_QuadraticSegment>())).p as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_QuadraticSegment),
            "::",
            stringify!(p)
        )
    );
}
#[doc = " A cubic Bezier curve."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_CubicSegment {
    pub _base: msdfgen_EdgeSegment,
    pub p: [msdfgen_Point2; 4usize],
}
#[test]
fn bindgen_test_layout_msdfgen_CubicSegment() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_CubicSegment>(),
        72usize,
        concat!("Size of: ", stringify!(msdfgen_CubicSegment))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_CubicSegment>(),
        8usize,
        concat!("Alignment of ", stringify!(msdfgen_CubicSegment))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_CubicSegment>())).p as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_CubicSegment),
            "::",
            stringify!(p)
        )
    );
}
#[doc = " Container for a single edge of dynamic type."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_EdgeHolder {
    pub edgeSegment: *mut msdfgen_EdgeSegment,
}
#[test]
fn bindgen_test_layout_msdfgen_EdgeHolder() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_EdgeHolder>(),
        4usize,
        concat!("Size of: ", stringify!(msdfgen_EdgeHolder))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_EdgeHolder>(),
        4usize,
        concat!("Alignment of ", stringify!(msdfgen_EdgeHolder))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_EdgeHolder>())).edgeSegment as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_EdgeHolder),
            "::",
            stringify!(edgeSegment)
        )
    );
}
#[doc = " A single closed contour of a shape."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_Contour {
    #[doc = " The sequence of edges that make up the contour."]
    pub edges: [u32; 3usize],
}
#[test]
fn bindgen_test_layout_msdfgen_Contour() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_Contour>(),
        12usize,
        concat!("Size of: ", stringify!(msdfgen_Contour))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_Contour>(),
        4usize,
        concat!("Alignment of ", stringify!(msdfgen_Contour))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Contour>())).edges as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Contour),
            "::",
            stringify!(edges)
        )
    );
}
#[doc = " Vector shape representation."]
#[repr(C)]
#[derive(Debug)]
pub struct msdfgen_Shape {
    #[doc = " The list of contours the shape consists of."]
    pub contours: [u32; 3usize],
    #[doc = " Specifies whether the shape uses bottom-to-top (false) or top-to-bottom (true) Y coordinates."]
    pub inverseYAxis: bool,
}
#[test]
fn bindgen_test_layout_msdfgen_Shape() {
    assert_eq!(
        ::std::mem::size_of::<msdfgen_Shape>(),
        16usize,
        concat!("Size of: ", stringify!(msdfgen_Shape))
    );
    assert_eq!(
        ::std::mem::align_of::<msdfgen_Shape>(),
        4usize,
        concat!("Alignment of ", stringify!(msdfgen_Shape))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Shape>())).contours as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Shape),
            "::",
            stringify!(contours)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<msdfgen_Shape>())).inverseYAxis as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(msdfgen_Shape),
            "::",
            stringify!(inverseYAxis)
        )
    );
}
pub type msdfgen_byte = ::std::os::raw::c_uchar;
pub const msdfgen_SegmentKind_LINEAR: msdfgen_SegmentKind = 0;
pub const msdfgen_SegmentKind_QUADRATIC: msdfgen_SegmentKind = 1;
pub const msdfgen_SegmentKind_CUBIC: msdfgen_SegmentKind = 2;
pub type msdfgen_SegmentKind = u32;
#[test]
fn __bindgen_test_layout_std_allocator_open0_msdfgen_Scanline_Intersection_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_msdfgen_Scanline_Intersection_close0_instantiation_1()
{
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_msdfgen_Scanline_Intersection_close0_instantiation_2()
{
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_msdfgen_EdgeHolder_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
#[test]
fn __bindgen_test_layout_std_allocator_open0_msdfgen_Contour_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<std_allocator>(),
        1usize,
        concat!(
            "Size of template specialization: ",
            stringify!(std_allocator)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<std_allocator>(),
        1usize,
        concat!(
            "Alignment of template specialization: ",
            stringify!(std_allocator)
        )
    );
}
