{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [ pkg-config clang lld ];
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    sea-orm-cli
    openssl
  ];

  RUST_BACKTRACE = 1;
}
