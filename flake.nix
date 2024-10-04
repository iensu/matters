{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk/master";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        name = "matters";
        version = "0.1.0";
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustBinaries = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        naersk-lib = pkgs.callPackage naersk {
          cargo = rustBinaries;
          rustc = rustBinaries;
        };
        nativeBuildInputs = [];
        cliPackage = naersk-lib.buildPackage {
          inherit name version nativeBuildInputs;

          root = builtins.path { path = ./.; inherit name; };
          cargoBuildOptions = x: x ++ [ "--package" name ];
          cargoTestOptions = x: x ++ [ "--package" name ];
          release = true;
        };

        wasmPackage = naersk-lib.buildPackage {
          src = ./.;
          cargoBuildOptions = x: x ++ [
            "--package" "${name}_wasm"
          ];
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
          copyLibs = true;
          copyBins = false;
          nativeBuildInputs = with pkgs; [ binaryen ];
          postInstall = ''
            # Optimize Wasm file size
            wasm-opt -Oz -o $out/lib/${name}.wasm $out/lib/${name}_wasm.wasm
            # Cleanup the lib directory
            find $out/lib -type f ! -name "${name}.wasm" -delete
          '';
        };

        sitePackage = pkgs.stdenv.mkDerivation rec {
          name = "example-site";
          src = ./.;
          installPhase = ''
            mkdir -p $out
            cp example-site/* $out/
            cp ${wasmPackage}/lib/* $out/
          '';
        };
        siteServer = pkgs.writeScriptBin "serve" ''
          ${pkgs.python3}/bin/python3 -m http.server --directory ${sitePackage}
        '';
      in
        {
          packages.cli = cliPackage;
          packages.wasm = wasmPackage;
          packages.site = sitePackage;
          defaultPackage = self.packages.${system}.cli;

          apps.matte = {
            type = "app";
            program = "${cliPackage}/bin/matte";
          };

          devShell = with pkgs; mkShell {
            inherit nativeBuildInputs;

            buildInputs = [
              rustBinaries
              binaryen     # Tools for optimizing wasm modules (wasm-* family of executables)
              wabt         # Tools for working with wasm text format (wasm2wat, wat2wasm, ...)
              wasmtime     # For running the wasm module through WASI (wasmtime --invoke <fn> file.wasm [...args])
              cargo-outdated

              siteServer
            ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        });
}
