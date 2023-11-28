{
  description = "Library to parse POST requests of traQ BOT event";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, fenix, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        toolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-S4dA7ne2IpFHG+EnjXfogmqwGyDFSRWFnJ8cy4KZr1k=";
        };
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        formatter = pkgs.nixpkgs-fmt;
        devShells.default = pkgs.stdenv.mkDerivation {
          name = "rust-env";
          nativeBuildInputs = with pkgs; [
            toolchain
            grcov # to collect coverage
            libiconv # required to build example `basic-with-axum`
          ];
        };
        devShells.validate = pkgs.stdenv.mkDerivation {
          name = "validate";
          nativeBuildInputs = with pkgs; [
            taplo
            nixpkgs-fmt
            action-validator
          ];
        };
      }
    );
}
