{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rust-bin.stable.latest.default
            rust-analyzer

            sfz
            elmPackages.elm
            elmPackages.elm-format
            elmPackages.elm-language-server

            ocaml
            dune_3
            ocamlPackages.ocaml-lsp
            ocamlPackages.ocamlformat

            go
            gopls

            deno

            mypy
            ruff
            python3
            python311Packages.python-lsp-server
          ];
        };

        formatter = nixpkgs-fmt;
      }
    );
}
