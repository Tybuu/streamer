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
              wayland
              wayland-protocols
              libxkbcommon
              libGL
              libdecor # For window decorations (title bar, etc.)
              dbus # For system-wide message bus communication
              at-spi2-atk # For the accessibility toolkit, a common hidden dependency
              (rust-bin.stable.latest.default.override
                {
                  extensions = ["rust-src" "rust-analyzer" "llvm-tools"];
                  targets = ["x86_64-pc-windows-msvc"];
                })
            ];

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libGL
            ];
          };
      }
    );
}
