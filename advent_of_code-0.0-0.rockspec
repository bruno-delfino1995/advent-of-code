rockspec_format = "3.0"

package = "advent_of_code"
version = "0.0-0"

source = {
  url = "git+ssh://git@github.com:bruno-delfino1995/advent-of-code.git",
  branch = "lua"
}

description = {
  homepage = "https://github.com/bruno-delfino1995/advent-of-code",
  license = "MIT"
}

dependencies = {
  "lua >= 5.1, < 5.5",
  "lua_cliargs >= 3.0"
}

build = {
   type = "builtin",
   modules = {
      main = "src/main.lua"
   }
}
