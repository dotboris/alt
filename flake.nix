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
      inherit (pkgs) lib;

      rustToolchain = pkgs.rust-bin.stable.latest.default;

      craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

      rustSrc = lib.cleanSourceWith {
        src = ./.;
        filter = craneLib.filterCargoSources;
      };
      src = with lib.fileset;
        toSource {
          root = ./.;
          fileset = unions [
            (fromSource rustSrc)
            ./README.md
            ./etc
          ];
        };

      commonArgs = {
        inherit src;
        strictDeps = true;
        buildInputs = lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      };
      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      alt = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
          doCheck = false; # runs through `nix flake check`

          postInstall = ''
            # shell hooks
            install -D etc/profile.d/alt.sh $out/etc/profile.d/alt.sh
            install -D \
              etc/fish/conf.d/alt.fish \
              $out/share/fish/vendor_conf.d/alt.fish

            # docs / man pages
            install -D README.md $out/share/doc/alt/README.md
            install -D target/release/man/*.1 -t $out/share/man/man1

            # completion
            install -D \
              target/release/completion/alt.bash \
              $out/share/bash-completion/completions/alt.bash
            install -D \
              target/release/completion/alt.fish \
              $out/share/fish/vendor_completions.d/alt.fish
            install -D \
              target/release/completion/_alt \
              $out/share/zsh/site-functions/_alt
          '';
        });
    in {
      packages.default = alt;
      apps.default = flake-utils.lib.mkApp {drv = alt;};

      formatter = pkgs.alejandra;
      checks = {
        inherit alt;
        clippy = craneLib.cargoClippy (commonArgs // {inherit cargoArtifacts;});
        test = craneLib.cargoTest (commonArgs
          // {
            inherit cargoArtifacts;
            src = with lib.fileset;
              toSource {
                root = ./.;
                fileset = unions [
                  (fromSource rustSrc)
                  ./tests/snapshots
                ];
              };
          });
        rustfmt = craneLib.cargoFmt {inherit src;};
        alejandra =
          pkgs.runCommand "alejandra" {
            buildInputs = [pkgs.alejandra];
          } ''
            alejandra -c ${./.}
            mkdir $out
          '';
        shellcheck =
          pkgs.runCommand "shellcheck" {
            buildInputs = [pkgs.fd pkgs.shellcheck];
          } ''
            fd . ${./.} -e .sh -e .bash -e .zsh -X shellcheck '{}'
            mkdir $out
          '';
        black =
          pkgs.runCommand "black" {
            buildInputs = [pkgs.python311Packages.black];
          } ''
            black --check ${./.}
            mkdir $out
          '';
        flake8 =
          pkgs.runCommand "flake8" {
            buildInputs = [pkgs.python311Packages.flake8];
          } ''
            flake8 --config ${./.}/setup.cfg ${./.}
            mkdir $out
          '';
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
        packages = with pkgs; [
          cargo-insta

          # Various supported shells for testing integrations
          bash
          fish
          zsh

          # Packaging tooling (in `ci/`)
          python311
          cargo-cross
          cargo-deb
        ];
      };
    });
}
