#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>

int main(){
    lua_State* L = luaL_newstate();
    luaL_openlibs(L);

    luaL_dofile(L,"motion.lua");

    lua_close(L);
    return 0;
}