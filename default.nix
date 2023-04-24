{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "xip";
  version = "0.5.0";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
