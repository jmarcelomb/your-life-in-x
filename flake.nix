{
  description = "Rust environment for ESP32-C3 development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" "llvm-tools-preview" ];
          targets = [ "riscv32imc-unknown-none-elf" ];
        };

        espTools = with pkgs; [
          cargo-binutils  # For objdump, nm, etc.
          cargo-generate  # Generate templates
          espflash        # Flash ESP32
          ldproxy         # Linker
        ];

        embeddedGraphicsSimulation = with pkgs; [
          SDL2
        ];

      in {
        devShells.default = pkgs.mkShell {
          name = "dev";
          buildInputs = [ rustToolchain ] ++ espTools ++ embeddedGraphicsSimulation;
        };
      });
}
