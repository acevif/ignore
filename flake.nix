{
  description = "Create `.gitignore` from GitHub/TopTal (gitignore.io) templates + your own patterns, all configured in `Ignorefile`";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    systems.url = "github:nix-systems/default";
  };

  outputs =
    inputs@{
      nixpkgs,
      nixpkgs-unstable,
      flake-parts,
      systems,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import systems;
      perSystem =
        { system, ... }:
        let
          pkgs = import nixpkgs { inherit system; };
          pkgs-unstable = import nixpkgs-unstable {
            inherit system;
            config.allowUnfree = true;
          };
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
          packages = {
            default = ignorePackage;
            ignore = ignorePackage;
          };

          devShells.default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                rustup
                rust-analyzer
                zsh
                opencode
              ]
              ++ (with pkgs-unstable; [
                codex
                claude-code
                gemini-cli
              ]);

            shellHook = ''
              exec zsh
            '';
          };
        };
    };
}
