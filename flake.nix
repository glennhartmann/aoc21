{
  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs;
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-compat, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        aoc21 = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          version = manifest.version;
          src = builtins.path { path = ./.; name = "aoc21"; };

          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "aoclib-rs-0.0.14" = "sha256-WRzigGCgqWQd9mFpFE1EiffAi57vD+M86WxKIHIAJMY=";
            };
          };
        };
        aoc21-shell = pkgs.mkShell {
          inputsFrom = [ aoc21 ];
          packages = with pkgs; [
            clippy
            rustfmt
          ];
        };
      in
      {
        packages = {
          inherit aoc21;
          default = aoc21;
        };
        devShells = {
          inherit aoc21-shell;
          default = aoc21-shell;
        };
      }
    );
}
