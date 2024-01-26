{
  description = "Switches between different versions of commands based on your current directory";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/stable";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        targets = pkgs.lib.optionals pkgs.stdenv.isLinux [
          # We package using musl on linux
          "x86_64-unknown-linux-musl"
        ];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      src = let
        # .snap files are snapshots used by the tests
        snapFilter = path: _type: builtins.match ".*\.snap$" path != null;
        srcFilter = path: type:
          (snapFilter path type) || (craneLib.filterCargoSources path type);
      in
        pkgs.lib.cleanSourceWith {
          src = craneLib.path ./.;
          filter = srcFilter;
        };
      commonArgs = {
        inherit src;
        strictDeps = true;
        buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      alt = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
          doCheck = false; # run through checks
        });
    in {
      packages.default = alt;
      apps.default = flake-utils.lib.mkApp {drv = alt;};

      formatter = pkgs.alejandra;
      checks = {
        inherit alt;
        clippy = craneLib.cargoClippy (commonArgs // {inherit cargoArtifacts;});
        test = craneLib.cargoTest (commonArgs // {inherit cargoArtifacts;});
        rustfmt = craneLib.cargoFmt {inherit src;};
        alejandra = pkgs.runCommand "alejandra" {} ''
          ${pkgs.alejandra}/bin/alejandra -c ${./.}
          mkdir $out
        '';
        shellcheck = pkgs.runCommand "shellcheck" {} ''
          ${pkgs.fd}/bin/fd . ${./.} \
            -e .sh -e .bash -e .zsh \
            -X ${pkgs.shellcheck}/bin/shellcheck '{}'
          mkdir $out
        '';
      };

      devShells.default = craneLib.devShell {
        # The Nix packages provided in the environment
        packages =
          (with pkgs; [
            rustToolchain

            # Various supported shells for testing integrations
            bash
            fish
            zsh

            # Packaging tooling (in `ci/`)
            (python311.withPackages (p: [
              p.black
              p.flake8
            ]))
            cargo-cross
            cargo-deb

            shellcheck
            cargo-insta
          ])
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      };
    });
}
