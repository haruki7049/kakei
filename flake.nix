{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    crane.url = "github:ipetkov/crane";
    flake-compat.url = "github:edolstra/flake-compat";
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

      perSystem =
        {
          pkgs,
          lib,
          system,
          ...
        }:
        let
          rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          overlays = [ inputs.rust-overlay.overlays.default ];
          craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust;

          src = lib.cleanSource ./.;

          cargoExtraArgs = "--locked --workspace";
          cargoArtifacts = craneLib.buildDepsOnly {
            inherit src;
          };
          kakei = craneLib.buildPackage {
            inherit src cargoArtifacts cargoExtraArgs;
            strictDeps = true;
            doCheck = true;
          };
          cargo-clippy = craneLib.cargoClippy {
            inherit src cargoArtifacts cargoExtraArgs;
            cargoClippyExtraArgs = "--verbose -- --deny warnings";
          };
          cargo-doc = craneLib.cargoDoc {
            inherit src cargoArtifacts cargoExtraArgs;
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
            projectRootFile = ".git/config";

            # Nix
            programs.nixfmt.enable = true;

            # Rust
            programs.rustfmt.enable = true;
            settings.formatter.rustfmt.command = "${rust}/bin/rustfmt";

            # SQL
            programs.sql-formatter.enable = true;
            programs.sql-formatter.dialect = "sqlite";

            # TOML
            programs.taplo.enable = true;

            # GitHub Actions
            programs.actionlint.enable = true;

            # Markdown
            programs.mdformat.enable = true;

            # ShellScript
            programs.shellcheck.enable = true;
            programs.shfmt.enable = true;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              # Compiler
              rust

              # Development Tools
              pkgs.sqlite # For debugging
              pkgs.sqlx-cli # For Database migration

              # Book Tools
              pkgs.mdbook
            ];
          };
        };
    };
}
