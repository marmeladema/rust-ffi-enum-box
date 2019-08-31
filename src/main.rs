#[derive(Debug)]
#[repr(C)]
pub enum SchemeType {
    Ip,
    Bytes,
    Int,
    Bool,
    Array(Box<SchemeType>),
    Map(Box<SchemeType>),
}

#[derive(Debug)]
pub struct Scheme {
    ty: Option<SchemeType>,
}

#[no_mangle]
pub extern "C" fn scheme_new() -> *mut Scheme {
    Box::into_raw(Box::new(Scheme { ty: None }))
}

#[no_mangle]
pub extern "C" fn scheme_free(scheme: *mut Scheme) {
    unsafe {
        Box::from_raw(scheme);
    }
}

#[no_mangle]
pub extern "C" fn scheme_set_type(scheme: *mut Scheme, ty: SchemeType) {
    unsafe {
        (*scheme).ty = Some(ty);
    }
}

#[no_mangle]
pub extern "C" fn scheme_print(scheme: *mut Scheme) {
    unsafe {
        println!("{:?}", (*scheme));
    }
}

extern "C" {
    fn scheme_ctest_01();
    fn scheme_ctest_02();
}

fn main() {
    println!("Hello, world!");
    println!(
        "sizeof(SchemeType) = {:?}",
        std::mem::size_of::<SchemeType>()
    );
    unsafe { scheme_ctest_01() };
    unsafe { scheme_ctest_02() };
}
