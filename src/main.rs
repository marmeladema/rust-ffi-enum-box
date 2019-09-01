use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug)]
pub enum SchemeType {
    Ip,
    Bytes,
    Int,
    Bool,
    Array(Box<SchemeType>),
    Map(Box<SchemeType>),
}

#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
enum CSchemeTypeTag {
    Ip,
    Bytes,
    Int,
    Bool,
    Array,
    Map,
}

#[repr(C)]
pub struct CSchemeType {
    tag: u8,
    data: Option<Box<SchemeType>>,
}

impl From<CSchemeType> for SchemeType {
    fn from(ty: CSchemeType) -> Self {
        match CSchemeTypeTag::try_from(ty.tag).unwrap() {
            CSchemeTypeTag::Ip => SchemeType::Ip,
            CSchemeTypeTag::Bytes => SchemeType::Bytes,
            CSchemeTypeTag::Int => SchemeType::Int,
            CSchemeTypeTag::Bool => SchemeType::Bool,
            CSchemeTypeTag::Array => SchemeType::Array(ty.data.unwrap()),
            CSchemeTypeTag::Map => SchemeType::Map(ty.data.unwrap()),
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
