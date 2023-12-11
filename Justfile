set positional-arguments

default:
  just --list

@setup:
  asdf install
  mix do deps.get, deps.compile

@prepare _:
  mix compile

@solve puzzle:
  mix solve "$1"
