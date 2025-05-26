{
    inputs = {
        nixpkgs.url = "nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = {self, flake-utils, nixpkgs}: 
        flake-utils.lib.eachDefaultSystem (system:
        let
            pkgs = import nixpkgs { inherit system; };
        in {
            devShell = pkgs.mkShell {
                packages = with pkgs; [
                    cargo
                    clippy
                    rustc
                    rustfmt
                    rustup
                    lld_19
                ];
                shellHook = ''
                export RUST_SRC_PATH=${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}
                zsh'';
            };
        }
    );
}
