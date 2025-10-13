{
  description = "demo-iced";

  nixConfig = {
    extra-substituters = [
      "https://mirrors.ustc.edu.cn/nix-channels/store"
    ];
    trusted-substituters = [
      "https://mirrors.ustc.edu.cn/nix-channels/store"
    ];
  };


  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-tools = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          # prioritize system clang, see https://github.com/zed-industries/zed/issues/7036
          # https://github.com/gfx-rs/gfx/issues/2309
          # https://mac.install.guide/commandlinetools/7
          shellHook = ''
            export PATH=/usr/bin:$PATH
          '';

          buildInputs = with pkgs; [
          ] ++ [
            trunk
            rust-tools
          ];
        };
      }
    );
}
