{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/master";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    crate2nix = {
      url = "github:kolloch/crate2nix";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, crate2nix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        crateName = "entropy";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        buildInputs = with pkgs; [ ];

        nativeBuildInputs = with pkgs; [
          nodejs-16_x
          yarn
          nixfmt
          pkgconfig
          rust-analyzer
          cargo-edit
          cargo-audit
          cargo-outdated
          wasm-pack
          (rust-bin.nightly.latest.default.override {
            extensions = [
              "rust-src"
              "cargo"
              "rustc"
              "rust-analysis"
              "rustfmt"
              "clippy"
            ];
            targets = ["wasm32-unknown-unknown"];
          })
        ];

      in
      {
        devShell = pkgs.mkShell
          ({
            inherit buildInputs nativeBuildInputs;
            RUST_BACKTRACE = 1;
          });
      });
}
