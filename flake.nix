{
  description = "rust-game-bevy";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-21.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, ... }:
    utils.lib.eachDefaultSystem (system:
      let
        bintools_lld = pkgs.wrapBintoolsWith {
          bintools = pkgs.llvmPackages_10.bintools;
        };
        inherit (lib) attrValues;
        mkPkgs = pkgs: extraOverlays:
          import pkgs {
            inherit system;
            config = { allowUnfree = true; };
          };
          pkgs = mkPkgs nixpkgs [ ];
          lib = nixpkgs.lib;
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustup
            rustc
            rust-analyzer
            rustfmt
            cargo
            lld
            netpbm
            feh
          ];
        };
      });
}
