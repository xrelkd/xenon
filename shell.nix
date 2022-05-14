{ pkgs ? import <nixpkgs>, ... }:

pkgs.mkShell rec {
  name = "dev-shell";

  buildInputs = with pkgs; [
    rustup

    tokei

    pkg-config

    openssl

    jq
    nixpkgs-fmt
    shfmt
    nodePackages.prettier
    shellcheck
  ] ++ lib.optional stdenv.hostPlatform.isDarwin [ darwin.apple_sdk.frameworks.Security libiconv ];

  shellHook = ''
    export NIX_PATH="nixpkgs=${pkgs.path}"
    export PATH=$PWD/dev-support/bin:$PATH
  '';
}
