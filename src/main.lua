local cli = require("cliargs")

-- this is called when the flag -v or --version is set
local function print_version()
  print("advent-of-code: version 0.0-0")
  os.exit(0)
end

cli:flag("-v, --version", "prints the program's version and exits", print_version)

local args, err = cli:parse(arg)

if not args and err then
  print(string.format('%s: %s; re-run with help for usage', cli.name, err))
  os.exit(1)
end
