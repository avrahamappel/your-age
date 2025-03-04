{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    { flake-utils
    , naersk
    , nixpkgs
    , fenix
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        naersk' = pkgs.callPackage naersk { };

        cargoDeps = pkgs.rustPlatform.importCargoLock {
          lockFile = ./Cargo.lock;
        };

        stableToolchain = (pkgs.fenix.stable.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
        ]);

        wasmToolchain = pkgs.fenix.combine [
          stableToolchain
          (pkgs.fenix.fromToolchainFile {
            dir = ./.;
            sha256 = "sha256-AJ6LX/Q/Er9kS15bn9iflkUwcgYqRQxiOIL2ToVAXaU=";
          })
        ];

        # A wrapper for Cargo that will always use deps in the Nix store
        wrappedCargo = pkgs.writeShellScriptBin "cargo" ''
          ${stableToolchain}/bin/cargo \
            --offline \
            --config 'source.crates-io.replace-with="cargoDeps"' \
            --config 'source.cargoDeps.directory="${cargoDeps}"' \
            "$@"
        '';
      in
      {
        defaultPackage = naersk'.buildPackage {
          src = ./.;
        };

        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            wrappedCargo
            trunk
            rust-analyzer
            wasmToolchain
          ];
        };
      }
    );
}
