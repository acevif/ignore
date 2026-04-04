{
  description = "Create `.gitignore` from GitHub/TopTal (gitignore.io) templates + your own patterns, all configured in `Ignorefile`";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
  };

  outputs =
    inputs@{
      flake-parts,
      systems,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;
      perSystem =
        { pkgs, ... }:
        let
          ignorePackage = pkgs.rustPlatform.buildRustPackage {
            pname = "ignore";
            version = "0.4.0";

            src = ./rust;

            cargoLock = {
              lockFile = ./rust/Cargo.lock;
            };

            meta = with pkgs.lib; {
              description = "Create `.gitignore` from `Ignorefile`";
              longDescription = ''
                Generate `.gitignore` from GitHub/TopTal (gitignore.io) templates and your own patterns,
                all configured in `Ignorefile`.
              '';
              homepage = "https://github.com/acevif/ignore";
              mainProgram = "ignore";
            };
          };
        in
        {
          formatter = pkgs.nixfmt-tree;

          packages = {
            default = ignorePackage;
            ignore = ignorePackage;
          };
        };
    };
}
