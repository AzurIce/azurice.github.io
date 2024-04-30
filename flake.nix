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
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    nur.url = github:nix-community/NUR;
  };

  outputs = { self, nur, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          shellHook = ''
          export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
          '';
          buildInputs = with pkgs; [
            mdbook
            mdbook-katex
            mdbook-mermaid
          ] ++ [
          ] ++ (with pkgs.darwin.apple_sdk.frameworks; pkgs.lib.optionals pkgs.stdenv.isDarwin [
            SystemConfiguration
          ]);
        };
      }
    );
}
