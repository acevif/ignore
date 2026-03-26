{
  description = "Development shell for ignore";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    llm-agents.url = "github:numtide/llm-agents.nix";
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
        { pkgs, inputs', ... }:
        {
          formatter = pkgs.nixfmt-tree;

          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.rustup
              pkgs.rust-analyzer
              pkgs.zsh
              inputs'.llm-agents.packages.codex
              inputs'.llm-agents.packages.claude-code
              inputs'.llm-agents.packages.gemini-cli
              inputs'.llm-agents.packages.ccusage
              inputs'.llm-agents.packages.opencode
            ];

            shellHook = ''
              exec zsh
            '';
          };
        };
    };

  # Use llm-agents.nix's binary cache for prebuilt agent packages.
  nixConfig = {
    extra-substituters = [ "https://cache.numtide.com" ];
    extra-trusted-public-keys = [ "niks3.numtide.com-1:DTx8wZduET09hRmMtKdQDxNNthLQETkc/yaX7M4qK0g=" ];
  };
}
