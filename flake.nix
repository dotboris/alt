{
  description = "Switches between different versions of commands based on your current directory";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
      inherit (pkgs) lib;
      craneLib = crane.mkLib pkgs;

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

      formatter = pkgs.writeShellScriptBin "fmt" ''
        ${pkgs.alejandra}/bin/alejandra .
      '';
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
        packages = [
          pkgs.cargo-insta
          pkgs.rust-analyzer

          # Various supported shells for testing integrations
          pkgs.bash
          pkgs.fish
          pkgs.zsh

          # Packaging tooling (in `ci/`)
          pkgs.python311
          pkgs.cargo-cross
          pkgs.cargo-deb
        ];

        # Ensure `rust-analyzer` has access to the rust source code.
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
