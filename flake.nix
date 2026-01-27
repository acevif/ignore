{
  description = "Rust dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, nixpkgs-unstable, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        pkgs-unstable = import nixpkgs-unstable {
          inherit system;
          config.allowUnfree = true;
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "ignore";
          version = "0.4.0";

          src = ./rust;

          cargoLock = {
            lockFile = ./rust/Cargo.lock;
          };

          meta = with pkgs.lib; {
            description = "Manage .gitignore file from Ignorefile";
            homepage = "https://github.com/acevif/ignore";
            mainProgram = "ignore";
          };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
            zsh
            opencode
          ] ++ (with pkgs-unstable; [
            codex
            claude-code
            gemini-cli
          ]);

          shellHook = ''
            exec zsh
          '';
        };
      });
}
