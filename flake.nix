{
  description = "blog.luelo.dev";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    astro-language-tools = {
      url = "github:withastro/language-tools";
      flake = false;
    };
    treefmt-nix = {
      url = "github:unionlabs/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    turbo = {
      url = "github:alexghr/turborepo.nix/v1.10.15";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ nixpkgs
    , flake-parts
    , treefmt-nix
    , astro-language-tools
    , ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];

      imports = [
        treefmt-nix.flakeModule
      ];

      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
        in
        {
          _module.args.pkgs = import nixpkgs {
            inherit system;
          };

          packages = {
            astro-lsp = pkgs.buildNpmPackage {
              pname = "astro-language-server";
              version = "2.5.5";

              src = astro-language-tools;

              npmDepsHash = "sha256-tuEfyePwlOy2/mOPdXbqJskO6IowvAP4DWg8xSZwbJw=";

              # The prepack script runs the build script, which we'd rather do in the build phase.
              npmPackFlags = [ "--ignore-scripts" ];
            };
          };

          devShells = {
            default = pkgs.mkShell {
              ASTRO_TELEMETRY_DISABLED = 1;
              buildInputs = with pkgs; [
                rnix-lsp
                nodejs
                nodePackages_latest.typescript-language-server
                nodePackages_latest.svelte-language-server
              ];
              nativeBuildInputs = [ config.treefmt.build.wrapper ]
                ++ (pkgs.lib.attrsets.attrValues config.treefmt.build.programs);
            };
          };

          treefmt = {
            projectRootFile = "flake.nix";
            programs.nixpkgs-fmt.enable = true;
          };
        };
    };
}
