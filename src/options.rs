use crate::Build;

impl Build {
    /// If defined, Pluto errors will use ANSI color codes.
    pub fn opt_use_colored_output(&mut self) -> &mut Self {
        self.0.define("PLUTO_USE_COLORED_OUTPUT", None);
        self
    }

    /// If defined, Pluto will exclude code snippets from error messages to make them shorter.
    pub fn opt_short_errors(&mut self) -> &mut Self {
        self.0.define("PLUTO_SHORT_ERRORS", None);
        self
    }

    /// If defined, Pluto won't assume that source files are UTF-8 encoded and restrict valid symbol names.
    pub fn opt_no_utf8(&mut self) -> &mut Self {
        self.0.define("PLUTO_NO_UTF8", None);
        self
    }

    /// If defined, Pluto will use a jumptable in the VM even if not compiled via GCC or Clang.
    /// This will generally improve runtime performance but can add minutes to compile time, depending on the setup.
    pub fn opt_force_jumptable(&mut self) -> &mut Self {
        self.0.define("PLUTO_FORCE_JUMPTABLE", None);
        self
    }

    /// If defined, Pluto won't imbue tables with a metatable by default.
    pub fn opt_no_default_table_metatable(&mut self) -> &mut Self {
        self.0.define("PLUTO_NO_DEFAULT_TABLE_METATABLE", None);
        self
    }

    // Warning Options

    /// If defined, the "global-shadow" warning is enabled by default.
    pub fn opt_warn_global_shadow(&mut self) -> &mut Self {
        self.0.define("PLUTO_WARN_GLOBAL_SHADOW", None);
        self
    }

    /// The list of globals covered by the "global-shadow" warning.
    /// Default value: `"table","string","arg"`
    pub fn opt_common_global_names(&mut self, names: &str) -> &mut Self {
        self.0.define("PLUTO_COMMON_GLOBAL_NAMES", Some(names));
        self
    }

    /// If defined, the "non-portable-code" warning is enabled by default.
    pub fn opt_warn_non_portable_code(&mut self) -> &mut Self {
        self.0.define("PLUTO_WARN_NON_PORTABLE_CODE", None);
        self
    }

    /// If defined, the "non-portable-bytecode" warning is enabled by default.
    pub fn opt_warn_non_portable_bytecode(&mut self) -> &mut Self {
        self.0.define("PLUTO_WARN_NON_PORTABLE_BYTECODE", None);
        self
    }

    /// If defined, the "non-portable-name" warning is enabled by default.
    pub fn opt_warn_non_portable_name(&mut self) -> &mut Self {
        self.0.define("PLUTO_WARN_NON_PORTABLE_NAME", None);
        self
    }

    // Compatibility Options

    /// If defined, Pluto will assign `pluto_` to new keywords which break previously valid Lua identifiers.
    /// So, for example, the `switch` keyword becomes `pluto_switch`. The `pluto_` variants are valid even if this is not defined.
    /// As of Pluto 0.7.0, scripts can individually set compatibility modes via `pluto_use`.
    pub fn opt_compatible_mode(&mut self) -> &mut Self {
        self.0.define("PLUTO_COMPATIBLE_MODE", None);
        self
    }

    /// If defined, Pluto's automatic keyword detection will more aggressively disable keywords if they're not used exactly as expected.
    /// This will help when scripters use these keywords as globals across files or before their definition.
    pub fn opt_paranoid_keyword_detection(&mut self) -> &mut Self {
        self.0.define("PLUTO_PARANOID_KEYWORD_DETECTION", None);
        self
    }

    // Optional Keywords

    /// If defined, Pluto will imply `pluto_use let` at the beginning of every script.
    /// Note that this keyword is deprecated as of 0.9.0.
    pub fn opt_use_let(&mut self) -> &mut Self {
        self.0.define("PLUTO_USE_LET", None);
        self
    }

    /// If defined, Pluto will imply `pluto_use const` at the beginning of every script.
    /// Note that this keyword is deprecated as of 0.9.0.
    pub fn opt_use_const(&mut self) -> &mut Self {
        self.0.define("PLUTO_USE_CONST", None);
        self
    }

    /// If defined, Pluto will imply `pluto_use global` at the beginning of every script.
    pub fn opt_use_global(&mut self) -> &mut Self {
        self.0.define("PLUTO_USE_GLOBAL", None);
        self
    }

    // Infinite Loop Prevention

    /// If defined, Pluto will attempt to prevent infinite loops.
    pub fn opt_ilp_enabled(&mut self) -> &mut Self {
        self.0.define("PLUTO_ILP_ENABLE", None);
        self
    }

    /// This is the maximum amount of backward jumps permitted in a singular loop block.
    /// If exceeded, the backward jump is ignored to escape the loop.
    pub fn opt_ilp_max_iterations(&mut self, iterations: u64) -> &mut Self {
        let s = iterations.to_string();
        self.0.define("PLUTO_ILP_MAX_ITERATIONS", Some(s.as_str()));
        self
    }

    /// If you want (i.e) `luaB_next` to reset iteration counters, define as `luaB_next`.
    pub fn opt_ilp_hook_function(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_ILP_HOOK_FUNCTION", Some(function));
        self
    }

    /// If defined, Pluto won't throw an error and instead just break out of the loop.
    pub fn opt_ilp_silent_break(&mut self) -> &mut Self {
        self.0.define("PLUTO_ILP_SILENT_BREAK", None);
        self
    }

    /// Allows you to customise how an ILP violation is raised to the runtime (or not) via injected C++ code.
    ///
    /// NOTE: This value is mutually exclusive with `PLUTO_ILP_SILENT_BREAK` and will be ignored if both are defined.
    ///
    /// Default value: `luaG_runerror(L, "infinite loop detected (exceeded max iterations: %d)", PLUTO_ILP_MAX_ITERATIONS);`
    pub fn opt_ilp_error(&mut self, code: &str) -> &mut Self {
        self.0.define("PLUTO_ILP_ERROR", Some(code));
        self
    }

    // Execution Time Limit

    /// If defined, Pluto will stop execution after a specified amount of time.
    pub fn opt_etl_enabled(&mut self) -> &mut Self {
        self.0.define("PLUTO_ETL_ENABLE", None);
        self
    }

    /// This is the maximum amount of nanoseconds the VM is allowed to run.
    pub fn opt_etl_nanos(&mut self, nanos: u64) -> &mut Self {
        let s = nanos.to_string();
        self.0.define("PLUTO_ETL_NANOS", Some(s.as_str()));
        self
    }

    /// This can be used to execute custom code when the time limit is exceeded and
    /// the VM is about to be terminated.
    ///
    /// Default value: `luaG_runerror(L, "Execution time limit exceeded");`
    pub fn opt_etl_timesup(&mut self, code: &str) -> &mut Self {
        self.0.define("PLUTO_ETL_TIMESUP", Some(code));
        self
    }

    // Memory Limit

    /// If defined, constraints the amount of memory that the VM is allowed to use in MB (64000000 = 64 MB).
    pub fn opt_memory_limit(&mut self, megabytes: u64) -> &mut Self {
        let s = megabytes.to_string();
        self.0.define("PLUTO_MEMORY_LIMIT", Some(s.as_str()));
        self
    }

    // VM Dump Options

    /// If defined, Pluto will print every VM instruction that is ran.
    /// Note that you can modify lua_writestring to redirect output.
    pub fn opt_vmdump_enabled(&mut self) -> &mut Self {
        self.0.define("PLUTO_VMDUMP", None);
        self
    }

    /// Opcodes listed in this structure are a blacklist. They not be printed when VM dumping.
    /// The values must be comma-separated, e.g. `"OP_LOADI, OP_LOADF"`.
    ///
    /// Valid opcodes: <https://github.com/PlutoLang/Pluto/blob/main/src/lopcodes.h#L197>
    pub fn opt_vmdump_ignore(&mut self, opcodes: &str) -> &mut Self {
        self.0.define("vmDumpIgnore", Some(opcodes));
        self
    }

    /// Opcodes listed in this structure are a whitelist. They are only printed when VM dumping.
    ///
    /// The values must be comma-separated, e.g. `"OP_LOADI, OP_LOADF"`.
    /// Call [`opt_vmdump_whitelist`][Self::opt_vmdump_whitelist] to use this option.
    ///
    /// Valid opcodes: <https://github.com/PlutoLang/Pluto/blob/main/src/lopcodes.h#L197>
    pub fn opt_vmdump_allow(&mut self, opcodes: &str) -> &mut Self {
        self.0.define("vmDumpAllow", Some(opcodes));
        self
    }

    /// If defined, Pluto will use [`opt_vmdump_allow`][Self::opt_vmdump_allow] instead of [`opt_vmdump_ignore`][Self::opt_vmdump_ignore].
    pub fn opt_vmdump_whitelist(&mut self) -> &mut Self {
        self.0.define("PLUTO_VMDUMP_WHITELIST", None);
        self
    }

    /// Defines under what circumstances the VM Dump is active. This must be valid C++ code.
    ///
    /// Default value: `true`
    pub fn opt_vmdump_cond(&mut self, code: &str) -> &mut Self {
        self.0.define("PLUTO_VMDUMP_COND", Some(code));
        self
    }

    // Content Moderation Options

    /// If defined, Pluto will not load compiled Lua or Pluto code.
    pub fn opt_disable_compiled(&mut self) -> &mut Self {
        self.0.define("PLUTO_DISABLE_COMPILED", None);
        self
    }

    /// If defined, the provided function will be called as bool(lua_State* L, const char* code).
    /// If it returns false, a Lua error is raised.
    ///
    /// NOTE: To have your Rust function be called, you must make it externally accessible by Pluto.
    /// An example of this would be:
    /// ```rs,no_run
    /// #[no_mangle]
    /// pub extern "C" fn contmod_on_load(lua: *mut lua_State, code: *const c_char) -> bool {
    ///     // ...
    /// }
    /// ```
    /// You can then call this function with the function name: `build.opt_load_hook("contmod_on_load")`.
    pub fn opt_load_hook(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_LOAD_HOOK", Some(function));
        self
    }

    /// If defined, the provided function will be called as bool(lua_State* L, const char* filename).
    /// If it returns false, a Lua error is raised. This will affect require and dofile.
    ///
    /// NOTE: Check the [`opt_load_hook`][Self::opt_load_hook] for an example of how to make your Rust hook callable from Pluto.
    pub fn opt_loadfile_hook(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_LOADFILE_HOOK", Some(function));
        self
    }

    /// It is possible to pass a reader function to the load function.
    /// Pluto currently offers no way to moderate code loaded like this,
    /// so you may define this to disable this method of code-loading.
    pub fn opt_disable_unmoderated_load(&mut self) -> &mut Self {
        self.0.define("PLUTO_DISABLE_UNMODERATED_LOAD", None);
        self
    }

    /// If defined, the provided function will be called as bool(lua_State* L, const char* path).
    /// If it returns false, a Lua eror is raised. This will affect require and package.loadlib.
    ///
    /// NOTE: Check the [`opt_load_hook`][Self::opt_load_hook] for an example of how to make your Rust hook callable from Pluto.
    pub fn opt_loadclib_hook(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_LOADCLIB_HOOK", Some(function));
        self
    }

    /// If defined, Pluto will not load the io library, and exclude os.remove and os.rename.
    /// It's highly suggested in most cases to define [`opt_no_os_execute`][Self::opt_no_os_execute] too, since os.execute can be used for filesystem access.
    /// It's suggested you implement [`opt_loadclib_hook`][Self::opt_loadclib_hook], etc, for even more powerful coverage.
    /// package.loadlib can still load other Pluto/Lua libraries and use their lua_CFunction objects.
    pub fn opt_no_filesystem(&mut self) -> &mut Self {
        self.0.define("PLUTO_NO_FILESYSTEM", None);
        self
    }

    /// Disables os.execute & io.popen.
    pub fn opt_no_os_execute(&mut self) -> &mut Self {
        self.0.define("PLUTO_NO_OS_EXECUTE", None);
        self
    }

    /// Eliminate any loading of any binaries. This removes package.loadlib and ffi.open and prevents 'require' from loading any C modules or shared libraries.
    pub fn opt_no_binaries(&mut self) -> &mut Self {
        self.0.define("PLUTO_NO_BINARIES", None);
        self
    }

    /// This can be used to execute custom code when a script attempts to load a binary that is not allowed (due to [`opt_no_binaries`][Self::opt_no_binaries]).
    ///
    /// Default value: `luaL_error(L, "binary modules cannot be loaded in this environment");`
    pub fn opt_no_binaries_fail(&mut self, code: &str) -> &mut Self {
        self.0.define("PLUTO_NO_BINARIES_FAIL", Some(code));
        self
    }

    /// If defined, luaL_openlibs will not include the `debug` library.
    pub fn opt_no_debuglib(&mut self) -> &mut Self {
        self.0.define("PLUTO_NO_DEBUGLIB", None);
        self
    }

    /// If defined, luaL_openlibs will not include the `coroutine` library.
    pub fn opt_no_corolib(&mut self) -> &mut Self {
        self.0.define("PLUTO_NO_COROLIB", None);
        self
    }

    /// If defined, all HTTP requests will fail.
    /// Note that the `socket` library can still be used to the same effect (with more effort).
    pub fn opt_disable_http_completely(&mut self) -> &mut Self {
        self.0.define("PLUTO_DISABLE_HTTP_COMPLETELY", None);
        self
    }

    /// If defined, the provided function will be called as bool(lua_State* L, const char* url).
    /// If it returns false, a Lua error is raised.
    /// Note that the `socket` library can still be used to the same effect (with more effort).
    ///
    /// NOTE: Check the [`opt_load_hook`][Self::opt_load_hook] for an example of how to make your Rust hook callable from Pluto.
    pub fn opt_http_request_hook(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_HTTP_REQUEST_HOOK", Some(function));
        self
    }

    /// If defined, the provided function will be called as bool(lua_State* L, const char* path)
    /// for any attempt to read a file's contents or metadata. The path will be UTF-8 encoded.
    /// If it returns false, a Lua error is raised.
    ///
    /// NOTE: Check the [`opt_load_hook`][Self::opt_load_hook] for an example of how to make your Rust hook callable from Pluto.
    pub fn opt_read_file_hook(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_READ_FILE_HOOK", Some(function));
        self
    }

    /// If defined, the provided function will be called as bool(lua_State* L, const char* path)
    /// for any attempt to write a file's contents or metadata. The path will be UTF-8 encoded.
    /// If it returns false, a Lua error is raised.
    ///
    /// NOTE: Check the [`opt_load_hook`][Self::opt_load_hook] for an example of how to make your Rust hook callable from Pluto.
    pub fn opt_write_file_hook(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_WRITE_FILE_HOOK", Some(function));
        self
    }

    /// If defined, the provided function will be called as bool(lua_State* L, void* addr)
    /// for any attempt to call a foreign function.
    /// If it returns false, a Lua error is raised.
    ///
    /// NOTE: Check the [`opt_load_hook`][Self::opt_load_hook] for an example of how to make your Rust hook callable from Pluto.
    pub fn opt_ffi_call_hook(&mut self, function: &str) -> &mut Self {
        self.0.define("PLUTO_FFI_CALL_HOOK", Some(function));
        self
    }

    // Performance Options

    /// If defined, disables the length cache.
    pub fn opt_disable_length_cache(&mut self) -> &mut Self {
        self.0.define("PLUTO_DISABLE_LENGTH_CACHE", None);
        self
    }

    /// If defined, disables table freezing.
    pub fn opt_disable_table_freezing(&mut self) -> &mut Self {
        self.0.define("PLUTO_DISABLE_TABLE_FREEZING", None);
        self
    }
}
