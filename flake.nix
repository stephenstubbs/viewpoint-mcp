{
  description = "viewpoint-mcp: MCP server for browser automation via Viewpoint";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            pkgs = import nixpkgs {
              inherit system;
              overlays = [
                rust-overlay.overlays.default
                self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          let
            rust = prev.rust-bin;
          in
          if builtins.pathExists ./rust-toolchain.toml then
            rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain then
            rust.fromRustupToolchainFile ./rust-toolchain
          else
            rust.stable.latest.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
              ];
            };
      };

      devShells = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # Rust toolchain
              rustToolchain
              cargo-edit
              cargo-workspaces

              # Build dependencies
              pkg-config

              # Browser for testing
              chromium
            ];

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };

            shellHook = ''
              echo "viewpoint-mcp development environment"
              echo "Rust: $(rustc --version)"
              echo ""
              echo "Commands:"
              echo "  cargo build          - Build the project"
              echo "  cargo test           - Run unit tests"
              echo "  cargo test --features integration - Run all tests (requires browser)"
              echo "  cargo run            - Run the MCP server"
              echo ""
            '';
          };
        }
      );

      packages = forEachSupportedSystem (
        { pkgs }:
        {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "viewpoint-mcp";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs; [
            ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

            # Skip integration tests during nix build (they require a browser)
            checkFlags = [ "--skip" "integration" ];

            meta = with pkgs.lib; {
              description = "MCP server for browser automation via Viewpoint";
              homepage = "https://github.com/stephenstubbs/viewpoint-mcp";
              license = licenses.mit;
              maintainers = [ ];
              mainProgram = "viewpoint-mcp";
            };
          };
        }
      );
    };
}
