{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        libPath = with pkgs; lib.makeLibraryPath [
          xdotool
          xorg.libX11
          dbus
        ];
      in
      {
        defaultPackage = naersk-lib.buildPackage {
          src = ./.;
          doCheck = true;
          pname = "easy-neovim-pop-shell-nav";
          nativeBuildInputs = [ pkgs.makeWrapper ];
          buildInputs = with pkgs; [
            dbus
            xdotool
            xorg.libX11
            pkg-config
          ];

          postInstall = ''
            wrapProgram "$out/bin/easy-neovim-pop-shell-nav" --prefix LD_LIBRARY_PATH : "${libPath}"
          '';

        };

        apps.default = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };
        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            pre-commit
            rustPackages.clippy
            pkg-config
            xdotool
            xorg.libX11
            dbus
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      });
}
