{ pkgs, }:

pkgs.runCommand "check-format"
{
  buildInputs = with pkgs; [
    treefmt
    fd
    nixpkgs-fmt
    nodePackages.prettier
    shellcheck
    shfmt
    sleek
  ];
} ''
  treefmt \
    --allow-missing-formatter \
    --fail-on-change \
    --no-cache \
    --formatters prettier nix shell \
    -C ${./..}

  # it worked!
  touch $out
''
