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
  ];
} ''
  treefmt \
    --allow-missing-formatter \
    --fail-on-change \
    --no-cache \
    --formatters prettier \
    --formatters nix \
    --formatters shell \
    -C ${./..}

  # it worked!
  touch $out
''
