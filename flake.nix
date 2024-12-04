{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    crane.url = "github:ipetkov/crane";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem = { pkgs, system, ... }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          overlays = [ inputs.rust-overlay.overlays.default ];
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust;
          src = craneLib.cleanCargoSource ./.;
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src;
          };
          kakei = craneLib.buildPackage {
            inherit src cargoArtifacts;
            strictDeps = true;

            doCheck = true;
          };
          cargo-clippy = craneLib.cargoClippy {
            inherit src cargoArtifacts;
            cargoClippyExtraArgs = "--verbose -- --deny warning";
          };
          cargo-doc = craneLib.cargoDoc {
            inherit src cargoArtifacts;
          };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system overlays;
          };

          packages = {
            inherit kakei;
            default = kakei;
            doc = cargo-doc;
          };

          checks = {
            inherit
              kakei
              cargo-clippy
              cargo-doc
              ;
          };

          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixpkgs-fmt.enable = true;
            programs.rustfmt.enable = true;
            programs.taplo.enable = true;
            programs.yamlfmt.enable = true;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              rust
            ];
          };
        };
    };
}
