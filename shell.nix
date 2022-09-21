{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  shellHook = ''
    export RUSTFLAGS=--cfg=web_sys_unstable_apis
  '';
}
