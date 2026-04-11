{ pkgs }:

pkgs.runCommand "check-format"
  {
    buildInputs = with pkgs; [
      fd
      nixfmt
      prettier
      shellcheck
      shfmt
      taplo
      treefmt
    ];
  }
  ''
    treefmt \
      --allow-missing-formatter \
      --fail-on-change \
      --no-cache \
      --formatters prettier \
      --formatters nix \
      --formatters toml \
      --formatters shell \
      -C ${./..}

    # it worked!
    touch $out
  ''
