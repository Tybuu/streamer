{
  description = "A Winit project with Wayland support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in {
        devShells.default = with pkgs;
          mkShell {
            buildInputs = [
              pkg-config
              libusb1
              (rust-bin.stable.latest.default.override
                {
                  extensions = ["rust-src" "rust-analyzer" "llvm-tools"];
                  targets = ["x86_64-pc-windows-msvc"];
                })
            ];
          };
      }
    );
}
