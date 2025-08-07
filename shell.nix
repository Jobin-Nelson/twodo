{
  pkgs ? import <nixpkgs> { },
}:
 pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
    bacon

    # Necessary for the openssl-sys crate:
    openssl
    pkg-config
  ];

  # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela.
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
