{
  description = "Description for the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    treefmt-nix = {
      url = "github:unionlabs/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, treefmt-nix, nixpkgs, rust-overlay, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        treefmt-nix.flakeModule
      ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          nightlyVersion = "2024-03-14";
          defaultChannel = "nightly-${nightlyVersion}";

          availableComponents = {
            rustc = "rustc";
            cargo = "cargo";
            rustfmt = "rustfmt";
            rust-std = "rust-std";
            rust-docs = "rust-docs";
            rust-analyzer = "rust-analyzer";
            clippy = "clippy";
            miri = "miri";
            rust-src = "rust-src";
            llvm-tools-preview = "llvm-tools-preview";
          };
          rust = pkgs.rust-bin.fromRustupToolchain {
            channel = defaultChannel;
            profile = "minimal";
            components = builtins.attrValues availableComponents;
          };

        in
        {
          _module.args = {
            pkgs = nixpkgs.legacyPackages.${system}.appendOverlays
              [
                rust-overlay.overlays.default
              ];
          };

          devShells.default = pkgs.mkShell {
            name = "devShell";
            buildInputs = [ rust pkgs.nil ];
            nativeBuildInputs = [ config.treefmt.build.wrapper ]
              ++ pkgs.lib.attrsets.attrValues config.treefmt.build.programs;
          };

          packages.default = pkgs.hello;
          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt = {
                enable = true;
                package = rust;
              };
              taplo = { enable = true; };
            };
            settings = {
              global = {
                excludes = [ ];
              };
            };
          };
        };
      flake = { };
    };
}
