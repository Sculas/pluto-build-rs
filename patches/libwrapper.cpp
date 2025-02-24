#include "lua.hpp"

bool plutow_wants_lib(const char** libs, int num_libs, const char* lib) {
    if (libs == nullptr || num_libs == 0) return true;
    for (int i = 0; i < num_libs; ++i) {
        if (strcmp(libs[i], lib) == 0) {
            return true;
        }
    }
    return false;
}

// Source: https://github.com/PlutoLang/Pluto/blob/main/src/linit.cpp#L64
LUALIB_API void plutow_openlibs(lua_State* L, const char** libs, int num_libs) {
    luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
    for (const Pluto::PreloadedLibrary* lib : Pluto::all_preloaded) {
        if (!plutow_wants_lib(libs, num_libs, lib->name)) continue;
        lua_pushcfunction(L, lib->init);
        lua_setfield(L, -2, lib->name);
    }
    lua_pop(L, 1);

#ifndef PLUTO_DONT_LOAD_ANY_STANDARD_LIBRARY_CODE_WRITTEN_IN_PLUTO
    const auto startup_code = R"EOC(
  pluto_use "0.6.0"
  
  class exception
      __name = "pluto:exception"
  
      function __construct(public what)
          local caller
          local i = 2
          while true do
              caller = debug.getinfo(i)
              if caller == nil then
                  error("exception instances must be created with 'pluto_new'", 0)
              end
              ++i
              if caller.name == "Pluto_operator_new" then
                  caller = debug.getinfo(i)
                  break
              end
          end
          self.where = $"{caller.short_src}:{caller.currentline}"
          error(self, 0)
      end
  
      function __tostring()
          return $"{self.where}: {tostring(self.what)}"
      end
  end
  
  function instanceof(a, b)
    return a instanceof b
  end
  )EOC";
    luaL_loadbuffer(L, startup_code, strlen(startup_code), "Pluto Standard Library");
    lua_call(L, 0, 0);
#endif
}