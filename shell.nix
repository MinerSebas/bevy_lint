{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    # For installing cargo-dylint
    pkg-config
    openssl
  ];
  RUSTC_VERSION = pkgs.lib.readFile ./rust-toolchain;
}
