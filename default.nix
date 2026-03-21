let
  pkgs = import <nixpkgs> { };
in
pkgs.callPackage ./nix/package.nix { }
