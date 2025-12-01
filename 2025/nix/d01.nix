let
  utils = import ./utils.nix;
  inherit (import <nixpkgs> { }) lib;
  inherit (utils.getInput "01") lines;

  deltas = map (
    line:
    let
      dist = lib.toInt (builtins.substring 1 (-1) line);
    in
    if (builtins.substring 0 1 line) == "L" then -dist else dist
  ) lines;

  crosses = (
    builtins.foldl'
      (
        acc: delta:
        let
          dial = utils.mod (acc.dial + delta) 100;
        in
        {
          inherit dial;
          part1 = acc.part1 + (if dial == 0 then 1 else 0);
          part2 =
            acc.part2
            + (
              if delta < 0 then
                (100 - acc.dial - delta) / 100 - (if acc.dial == 0 then 1 else 0)
              else
                (acc.dial + delta) / 100
            );
        }
      )
      {
        dial = 50;
        part1 = 0;
        part2 = 0;
      }
      deltas
  );
in
{
  inherit (crosses) part1 part2;
}
