{
  description = "A rust project";

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    fenix,
    naersk,
    ...
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlay];
      };
      rustChannel = pkgs.rustChannelOf {rustToolchain = ./rust-toolchain;};
      naersk' = pkgs.callPackage naersk {};
    in {
      defaultPackage =
        (naersk.lib.${system}.override {
          inherit (fenix.packages.${system}.minimal) cargo rustc;
        })
        .buildPackage {src = ./.;};
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          (fenix.packages.${system}.fromToolchainFile {
            dir = ./.;
            sha256 = "sha256-KXx+ID0y4mg2B3LHp7IyaiMrdexF6octADnAtFIOjrY=";
          })
          fenix.packages.${system}.rust-analyzer
          alejandra
        ];
      };
    });
}
