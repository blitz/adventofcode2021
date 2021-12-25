{
  description = "Advent of Code 2021";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };
  
  outputs = { self, nixpkgs, utils, naersk }:

    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in {
      # `nix build`
      packages.adventofcode2021 = naersk-lib.buildPackage {
        pname = "adventofcode2021";
        root = ./ws;
      };
      defaultPackage = self.packages."${system}".adventofcode2021;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo ];
      };
    });
}
