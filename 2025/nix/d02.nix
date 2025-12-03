let
  utils = import ./utils.nix;
  inherit (import <nixpkgs> { }) lib;
  inherit (utils.getInput "02") input;

  first = [
    [
      2
      1
    ]
    [
      4
      2
    ]
    [
      6
      3
    ]
    [
      8
      4
    ]
    [
      10
      5
    ]
  ];
  second = [
    [
      3
      1
    ]
    [
      5
      1
    ]
    [
      6
      2
    ]
    [
      7
      1
    ]
    [
      9
      3
    ]
    [
      10
      2
    ]
  ];
  third = [
    [
      6
      1
    ]
    [
      10
      1
    ]
  ];

  ranges = map (
    range:
    let
      split = lib.strings.splitString "-" range;
    in
    {
      start = lib.toInt (builtins.elemAt split 0);
      end = lib.toInt (builtins.elemAt split 1);
    }
  ) (lib.strings.splitString "," input);

  sum =
    sets:
    builtins.foldl' (
      acc: set:
      let
        len = builtins.elemAt set 1;
        repetitions = (builtins.elemAt set 0) / len;
        power = utils.pow 10 len;
        step = builtins.foldl' (acc: _: acc * power + 1) 0 (lib.range 0 (repetitions - 1)) + 0.0;
        invalid_start = step * (power / 10);
        invalid_end = step * (power - 1);
      in
      acc
      + (builtins.foldl' (
        acc: range:
        let
          lower = lib.max (utils.nextMultipleOf range.start step) invalid_start;
          upper = lib.min range.end invalid_end;
        in
        acc
        + (
          if lower <= upper then
            let
              n = builtins.floor (upper - lower) / builtins.floor step;
              m = n * (n + 1) / 2;
            in
            lower * (n + 1) + step * m
          else
            0
        )
      ) 0 ranges)
    ) 0 sets;

  part1 = sum first;
  part2 = part1 + sum second - sum third;
in
{
  part1 = builtins.toString (builtins.floor part1);
  part2 = builtins.toString (builtins.floor part2);
}
