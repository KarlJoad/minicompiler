{ pkgs ? import <nixpkgs> {} }:


pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    rls
    rustup
    rustracer
    rustfmt
    clippy

    bashInteractive
  ];
}
