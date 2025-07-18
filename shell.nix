{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ 
  rustc 
  cargo 
  gcc 
  rustfmt 
  clippy
  pkg-config
  rust-analyzer
#  openssl
#  openssl.dev
#  sqlite 
  ];

buildInputs = with pkgs; [ openssl ];

  # Certain Rust tools won't work without this
  # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
#  OPENSSL_DIR = pkgs.openssl.dev;
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;
#  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}
