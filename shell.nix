{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "aliendro-aoc24";
  buildInputs = with pkgs; [
    cargo
    rust-analyzer
    clippy
  ];
}
