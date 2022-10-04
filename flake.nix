{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, flake-utils, naersk, nixpkgs, }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          stdenv.hostPlatform.rustc.config = "wasm32-unknown-unknown";
        };

        # naersk' = pkgs.callPackage naersk { };

        deps = with pkgs; [ cargo rustc trunk ];

      in
      rec {
        # For `nix build` & `nix run`:
        defaultPackage = pkgs.rustPlatform.buildRustPackage
          {
            src = ./.;
            name = "your-age";
            nativeBuildInputs = deps;
            cargoLock = { lockFile = ./Cargo.lock; };
            buildPhase = ''
              trunk build --release
            '';
          };

        # For `nix develop`:
        devShell = pkgs.mkShell
          {
            nativeBuildInputs = deps;
          };

      }
    );
}
