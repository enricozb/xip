{ pkgs ? import <nixpkgs> { } }:

pkgs.rustPlatform.buildRustPackage rec {
  pname = "clip";
  version = "0.2.0";
  src = ./.;

  cargoLock = { lockFile = ./Cargo.lock; };

  buildInputs = with pkgs; [ gnutar zip ];
}
