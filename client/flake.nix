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
              libusb1
              libdecor
              dbus
              at-spi2-atk

              pipewire
              pipewire.jack
              alsa-lib
              alsa-plugins
              rtkit
              (rust-bin.stable.latest.default.override
                {
                  extensions = ["rust-src" "rust-analyzer" "llvm-tools"];
                })
            ];

            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.libGL
              pkgs.pipewire
              pkgs.pipewire.jack
              pkgs.alsa-lib
              pkgs.alsa-plugins
            ];
            ALSA_PLUGIN_DIR = "${pkgs.alsa-plugins}/lib/alsa-lib";
          };
      }
    );
}
