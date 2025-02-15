#[no_mangle]
pub extern "C" fn contmod_on_load(
    _lua: *mut mlua::ffi::lua_State,
    code: *const std::os::raw::c_char,
) -> bool {
    let code = unsafe { std::ffi::CStr::from_ptr(code) }.to_str().unwrap();
    println!("contmod_on_load: {code}");
    false
}

#[test]
fn test_pluto_hook() {
    let lua = mlua::Lua::new();
    match lua.load(r#"load("os.exit()")"#).set_name("hook test").exec() {
        Ok(_) => panic!("should have errored"),
        Err(mlua::Error::RuntimeError(e)) => println!("hook catched: {e}"),
        Err(e) => panic!("unexpected error: {e}"),
    }
}

#[test]
fn test_pluto_ilp() {
    let lua = mlua::Lua::new();
    match lua.load("while true do end").set_name("ilp test").exec() {
        Ok(_) => panic!("should have errored"),
        Err(mlua::Error::RuntimeError(e)) => println!("ilp catched: {e}"),
        Err(e) => panic!("unexpected error: {e}"),
    }
}
