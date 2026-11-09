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
    notist.url = "github:AzurIce/notist";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, notist, ... }:
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
          # Keep Nix tools first: the host Python may not include Pillow.
          packages = with pkgs; [
            typst
            just
            nodejs
            (python3.withPackages (ps: [ ps.pillow ]))
          ] ++ pkgs.lib.optional (builtins.hasAttr system notist.packages)
            notist.packages.${system}.default;

          buildInputs = with pkgs; [
          ] ++ [
            trunk
            rust-tools
          ];
        };
      }
    );
}
