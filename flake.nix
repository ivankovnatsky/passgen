{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/8a36010652b4571ee6dc9125cec2eaebc30e9400";
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
            config = {
              allowUnfree = true;
            };
          };

          # Latest stable Rust with minimal components
          rust = pkgs.rust-bin.stable.latest.minimal.override {
            extensions = [ "rust-src" "clippy" "rustfmt" ];
          };

          # Define the package
          passgen = pkgs.rustPlatform.buildRustPackage {
            pname = "passgen";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            buildInputs = [ ] ++ (if pkgs.stdenv.isDarwin then [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.libiconv
            ] else [ ]);

            meta = with pkgs.lib; {
              description = "A tool for generating secure passwords in Apple-like format";
              homepage = "https://github.com/ivankovnatsky/passgen";
              license = licenses.mit;
            };
          };

        in
        {
          # Expose the package
          packages.default = passgen;
          packages.passgen = passgen;

          # Development shell
          devShells.default = pkgs.mkShell {
            buildInputs = [
              rust
              pkgs.rust-analyzer
            ] ++ (if pkgs.stdenv.isDarwin then [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.libiconv
            ] else [ ]);

            shellHook = '''';
          };
        }
      );
} 
