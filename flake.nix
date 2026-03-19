{
  description = "The cyberpunk city powered by your kernel metrics";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource (craneLib.path ./.);

        # Common arguments can be set here to avoid repetition
        commonArgs = {
          inherit src;
          strictDeps = true;

          buildInputs = with pkgs; [
            # Add additional build inputs here
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via attic) between runs.
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        metropolis = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit metropolis;

          # Run tests with cargo-nextest or cargo test
          metropolis-tests = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
          });
        };

        packages.default = metropolis;

        apps.default = flake-utils.lib.mkApp {
          drv = metropolis;
        };

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # Additional dev-shell-only packages can be added here
          packages = with pkgs; [
            rust-analyzer
          ];
        };
      });
}
