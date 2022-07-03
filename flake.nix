{
  description = "Xenon - Some Useful Tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils }:
    (flake-utils.lib.eachDefaultSystem

      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlay ];
          };
        in
        rec {
          packages.xenon = pkgs.callPackage ./default.nix { };
          defaultPackage = packages.xenon;
          apps.default = flake-utils.lib.mkApp {
            drv = packages.xenon;
            exePath = "/bin/xenon";
          };
          devShell = pkgs.callPackage ./shell.nix { };

        })) // {
      overlay = final: prev: {
        xenon = final.callPackage ./default.nix { };
      };
    };
}
