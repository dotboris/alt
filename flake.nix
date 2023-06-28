{
  description =
    "Switches between different versions of commands based on your current directory";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";
    rust-overlay.url = "github:oxalica/rust-overlay/stable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Overlays enable you to customize the Nixpkgs attribute set
        overlays = [
          # Makes a `rust-bin` attribute available in Nixpkgs
          (import rust-overlay)
          # Provides a `rustToolchain` attribute for Nixpkgs that we can use to
          # create a Rust environment
          (self: super: {
            rustToolchain = super.rust-bin.stable.latest.default.override {
              targets = pkgs.lib.optionals pkgs.stdenv.isLinux [
                # We package using musl on linux
                "x86_64-unknown-linux-musl"
              ];
            };
          })
        ];
        pkgs = import nixpkgs { inherit system overlays; };

      in
      {
        formatter = pkgs.nixpkgs-fmt;

        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "alt";
          src = ./.;
          cargoLock = { lockFile = ./Cargo.lock; };

          # TODO: integrations test fail. It's likely something to do with the
          # PATH manipulation or the lack of the tmp dir.
          doCheck = false;

          nativeBuildInputs = [ pkgs.installShellFiles ];
          postInstall = ''
            target=${pkgs.rust.toRustTargetSpec pkgs.stdenv.hostPlatform}
            releaseDir=target/$target/$cargoBuildType

            installManPage "$releaseDir/man/"*.1

            installShellCompletion --bash "$releaseDir/completion/alt.bash"
            installShellCompletion --fish "$releaseDir/completion/alt.fish"
            installShellCompletion --zsh "$releaseDir/completion/_alt"
          '';
        };

        devShells.default = pkgs.mkShell {
          # The Nix packages provided in the environment
          packages = (with pkgs; [
            # The usual suite for rust tools including cargo, Clippy,
            # cargo-fmt rustdoc, rustfmt, and other tools.
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
          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin
            (with pkgs; [ libiconv ]);
        };
      });
}
