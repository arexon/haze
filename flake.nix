{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      inherit (pkgs) lib;

      craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);

      commonArgs = {
        src = lib.cleanSourceWith {
          src = lib.cleanSource ./.;
          filter = path: type: let
            pathStr = toString path;
            isTestSource = builtins.match ".*/tests/.*" pathStr != null;
            isCargoSource = craneLib.filterCargoSources path type;
          in
            isCargoSource || isTestSource;
        };
        strictDeps = true;
      };

      haze = craneLib.buildPackage (
        commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;

          doCheck = true;

          meta = with pkgs.lib; {
            description = "Dead simple world management tool for Minecraft Bedrock.";
            homepage = "https://github.com/arexon/haze";
            license = licenses.mit;
            mainProgram = "haze";
          };
        }
      );
    in {
      checks = {
        inherit haze;
      };

      formatter = pkgs.alejandra;

      packages = {
        inherit haze;
        default = haze;
      };

      apps.default = flake-utils.lib.mkApp {
        drv = haze;
      };

      devShells.default = craneLib.devShell {
        packages = with pkgs; [
          taplo
          dprint
          cargo-insta
        ];
      };
    });
}
