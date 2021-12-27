{
  description = "Advent of Code 2021";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-compat-ci.url = "github:hercules-ci/flake-compat-ci";

    mozillapkgs = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, naersk, mozillapkgs, flake-compat, flake-compat-ci }:
    {
      # For Hercules CI.
      ciNix = flake-compat-ci.lib.recurseIntoFlakeWith {
        flake = self;

        # Optional. Systems for which to perform CI.
        # By default, every system attr in the flake will be built.
        # Example: [ "x86_64-darwin" "aarch64-linux" ];
        systems = [ "x86_64-linux" ];
      };

    } //
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";


      # Get a specific rust version
      mozilla = pkgs.callPackage (mozillapkgs + "/package-set.nix") {};
      rust = (mozilla.rustChannelOf {
        date = "2020-12-24"; # get the current date with `date -I`
        channel = "nightly";
        sha256 = "sha256-ltQYo4AYmpD4dJV9WXDf0VSwj2cxilS1hrjTsj7Fcng=";
      }).rust;

      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };
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
