{
  description =
    "Switches between different versions of commands based on your current directory";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-23.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
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
        formatter = pkgs.nixpkgs-fmt;
      });
}
