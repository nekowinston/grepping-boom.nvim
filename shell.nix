{
  pkgs ? import <nixpkgs> {
    config = { };
    overlays = [ ];
  },
}:
let
  inherit (pkgs) lib;
  inherit (pkgs.stdenv) isDarwin isLinux;
  apple_frameworks = pkgs.darwin.apple_sdk.frameworks;
in
pkgs.mkShell {
  nativeBuildInputs =
    with pkgs;
    [
      cargo
      rustc
      clippy
      rustfmt
      rust-analyzer
      pkg-config

      alsaLib
    ]
    ++ lib.optionals isLinux [ ]
    ++ lib.optionals isDarwin [ ];

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
