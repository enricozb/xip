{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "clip";
  version = "0.4.1";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };
}
