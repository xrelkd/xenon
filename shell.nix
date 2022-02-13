{ pkgs ? import <nixpkgs>, ... }:

pkgs.mkShell rec {
  name = "dev-shell";

  buildInputs = with pkgs; [
    rustup

    tokei

    jq
    nixpkgs-fmt
    shfmt
    nodePackages.prettier
    shellcheck
  ];

  shellHook = ''
    export NIX_PATH="nixpkgs=${pkgs.path}"
    export PATH=$PWD/dev-support/bin:$PATH
  '';
}
