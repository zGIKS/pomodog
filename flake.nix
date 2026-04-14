{
  description = "A cute pomodoro CLI with a dog friend";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        
        naersk' = pkgs.callPackage naersk {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };
      in
      {
        packages.default = naersk'.buildPackage {
          pname = "pomodog";
          version = "0.1.0";
          src = ./.;

          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ ];

          meta = with pkgs.lib; {
            description = "A cute pomodoro CLI with a dog friend";
            homepage = "https://github.com/zGIKS/pomodoro-cli";
            license = licenses.mit;
            maintainers = [ ];
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            pkg-config
          ];

          shellHook = ''
            echo "Welcome to the pomodog development environment!"
            echo "Rust version: $(rustc --version)"
          '';
        };
      }
    );
}
