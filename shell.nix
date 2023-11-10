let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rustChannel = pkgs.rustChannelOf {
     channel = "stable";
  };
  rust = (rustChannel.rust.override {
    targets = [
      "wasm32-unknown-unknown" # required for the web-app
    ];
    extensions = ["rust-src" "rust-analysis"];
  });
in
  with pkgs;
  mkShell {
    buildInputs = [
      SDL2
      rust
      cargo
      rustfmt
      rust-analyzer
      clippy
    ];
    RUST_BACKTRACE = 1;
}
