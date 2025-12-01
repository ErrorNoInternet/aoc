let
  inherit (import <nixpkgs> { }) lib;
in
{
  mod =
    a: b:
    let
      r = a - (b * (a / b));
    in
    if r < 0 then r + b else r;

  getInput =
    day:
    let
      input = lib.trim (builtins.readFile ../inputs/d${day}.txt);
    in
    {
      inherit input;
      lines = lib.strings.splitString "\n" input;
    };
}
