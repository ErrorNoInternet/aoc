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

  pow = lib.fix (
    self: base: power:
    if power != 0 then base * (self base (power - 1)) else 1
  );

  nextMultipleOf = n: multiple: builtins.ceil (n / multiple) * multiple;

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
