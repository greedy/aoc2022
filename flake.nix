{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, naersk, rust-overlay }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; overlays = [ rust-overlay.overlays.default ]; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; with rust-bin.stable.latest;
        mkShell {
          buildInputs =
            [ rust-bin.stable.latest.default rust-analyzer clippy ] # cargo rustc rustfmt pre-commit rustPackages.clippy rust-analyzer ]
            ++ nixpkgs.lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [ Security OpenGL ApplicationServices AppKit ]);
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
