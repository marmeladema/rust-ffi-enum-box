#[derive(Debug)]
pub enum SchemeType {
    Ip,
    Bytes,
    Int,
    Bool,
    Array(Box<SchemeType>),
    Map(Box<SchemeType>),
}

#[repr(C)]
pub enum CSchemeType {
    Ip,
    Bytes,
    Int,
    Bool,
    Array(*mut SchemeType),
    Map(*mut SchemeType),
}

impl From<CSchemeType> for SchemeType {
    fn from(ty: CSchemeType) -> Self {
        match ty {
            CSchemeType::Ip => SchemeType::Ip,
            CSchemeType::Bytes => SchemeType::Bytes,
            CSchemeType::Int => SchemeType::Int,
            CSchemeType::Bool => SchemeType::Bool,
            CSchemeType::Array(arr) => SchemeType::Array(unsafe { Box::from_raw(arr) }),
            CSchemeType::Map(map) => SchemeType::Map(unsafe { Box::from_raw(map) }),
        }
    }
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
pub extern "C" fn scheme_set_type(scheme: *mut Scheme, ty: CSchemeType) {
    unsafe {
        (*scheme).ty = Some(ty.into());
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
    println!(
        "sizeof(CSchemeType) = {:?}",
        std::mem::size_of::<CSchemeType>()
    );
    unsafe { scheme_ctest_01() };
    unsafe { scheme_ctest_02() };
}
