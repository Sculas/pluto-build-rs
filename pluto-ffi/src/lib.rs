// Due to the way linking works in Rust (dependencies before build script),
// we have to use a macro to do all of this inside of the crate itself.

/// Load all or the specified Pluto libraries into the given Lua state.
/// This should not be called more than once and is inherently unsafe.
#[macro_export]
macro_rules! load_libraries {
    ($lua:expr, $libs:expr) => {unsafe {
        $crate::load_libraries!(@ffi_import);
        let libs: &[$crate::PlutoLibrary] = $libs;
        let libs = libs.iter().map(|lib| lib.ffi_name()).collect::<Vec<_>>();
        let libs = libs.iter().map(|lib| lib.as_ptr()).collect::<Vec<_>>();
        $lua.exec_raw::<()>((), |state| plutow_openlibs(state, libs.as_ptr(), libs.len() as _))
    }};
    ($lua:expr) => {unsafe {
        $crate::load_libraries!(@ffi_import);
        $lua.exec_raw::<()>((), |state| plutow_openlibs(state, std::ptr::null(), 0))
    }};
    (@ffi_import) => {
        extern "C" {
            #[allow(non_camel_case_types, non_snake_case)]
            pub fn plutow_openlibs(
                L: *mut mlua::ffi::lua_State,
                libs: *const *const std::os::raw::c_char,
                num_libs: std::os::raw::c_int,
            );
        }
    };
}

/// PlutoLibrary defines the available Pluto libraries that can be loaded.
pub enum PlutoLibrary {
    Crypto,
    Json,
    Base32,
    Base64,
    Assert,
    Vector3,
    Url,
    Star,
    Cat,
    Http,
    Scheduler,
    Socket,
    BigInt,
    Xml,
    Regex,
    FFI,
    Canvas,
}

impl PlutoLibrary {
    pub fn ffi_name(&self) -> std::ffi::CString {
        std::ffi::CString::new(match self {
            Self::Crypto => "crypto",
            Self::Json => "json",
            Self::Base32 => "base32",
            Self::Base64 => "base64",
            Self::Assert => "assert",
            Self::Vector3 => "vector3",
            Self::Url => "url",
            Self::Star => "*",
            Self::Cat => "cat",
            Self::Http => "http",
            Self::Scheduler => "scheduler",
            Self::Socket => "socket",
            Self::BigInt => "bigint",
            Self::Xml => "xml",
            Self::Regex => "regex",
            Self::FFI => "ffi",
            Self::Canvas => "canvas",
        })
        .unwrap()
    }
}
