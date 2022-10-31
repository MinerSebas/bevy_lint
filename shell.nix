{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    pkg-config
    openssl
  ];
  RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
}
