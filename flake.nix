{
  inputs.nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
      eachSystem = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.stdenv.system}.oxi ];

          nativeBuildInputs = with pkgs; [
            clippy
            rust-analyzer
            rustfmt
          ];

          env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };
      });

      packages = eachSystem (
        pkgs:
        let
          self' = self.packages.${pkgs.stdenv.system};
          src = pkgs.nix-gitignore.gitignoreSource [ ] ./.;
          version = "0.1.0";
        in
        {
          default = pkgs.vimUtils.buildVimPlugin {
            pname = "grepping-boom.nvim";
            inherit version src;

            postInstall = ''
              mkdir -p $out/lua
              cp -r ${self'.oxi}/lib/* $out/lua/grepping-boom.so
            '';
          };
          oxi = pkgs.rustPlatform.buildRustPackage {
            name = "grepping-boom_oxi";
            inherit version src;

            buildInputs = [ pkgs.alsaLib ];
            nativeBuildInputs = [ pkgs.pkg-config ];

            cargoLock.lockFile = ./Cargo.lock;
          };
        }
      );
    };
}
