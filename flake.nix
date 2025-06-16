{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        buildInputs = with pkgs; [
          dbus
        ];

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      in
      {

        devShells = {
          default = pkgs.mkShell {
            packages = with pkgs; [
              bacon
              cargo
              clippy
              rustc
              rustfmt
              rust-analyzer
            ] ++ buildInputs ++ nativeBuildInputs;
          };
        };
      });
}
