{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    let
      inherit (builtins) fromTOML readFile;
      argon2cliCargoToml = fromTOML (readFile ./argon2cli/Cargo.toml);

      mkArgon2cli = { lib, rustPlatform, ... }: rustPlatform.buildRustPackage {
        inherit (argon2cliCargoToml.package) name version;

        src = lib.cleanSource ./.;

        doCheck = true;

        cargoLock.lockFile = ./Cargo.lock;
      };
    in
    {
      overlays = rec {
        argon2cli = final: prev: {
          argon2cli = prev.callPackage mkArgon2cli { };
        };
        default = argon2cli;
      };
    }
    //
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        argon2cli = pkgs.callPackage mkArgon2cli { };
      in
      {
        # Executes by `nix build .#<name>`
        packages = {
          inherit argon2cli;
          default = argon2cli;
        };
        # the same but deprecated in Nix 2.7
        defaultPackage = self.packages.${system}.default;

        # Executes by `nix run .#<name> -- <args?>` 
        apps = {
          argon2cli = {
            type = "app";
            program = "${argon2cli}/bin/argon2cli";
          };
          default = self.apps.${system}.argon2cli;
        };
        # Executes by `nix run . -- <args?>`
        # the same but deprecated in Nix 2.7
        defaultApp = self.apps.${system}.default;

        # Used by `nix develop`
        devShell = with pkgs; mkShell {
          packages = [ cargo rustc rustfmt clippy rust-analyzer ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
