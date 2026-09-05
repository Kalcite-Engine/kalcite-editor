{
  description = "Kalcite Editor";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          runtimeLibraries = with pkgs; [
            libGL libxkbcommon wayland xorg.libX11 xorg.libXcursor xorg.libXfixes
            xorg.libXi xorg.libXrandr xorg.libXrender xorg.libxcb
          ];
        in rec {
          kalcite-editor = pkgs.rustPlatform.buildRustPackage {
            pname = "kalcite-editor";
            version = "0.14.0";
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
              allowBuiltinFetchGit = true;
            };
            nativeBuildInputs = [ pkgs.makeWrapper pkgs.pkg-config ];
            buildInputs = runtimeLibraries;
            postInstall = ''
              $out/bin/kalcite-editor-info linux $out
            '';
            postFixup = ''
              wrapProgram $out/bin/kalcite-editor \
                --prefix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath runtimeLibraries}
            '';
          };
          default = kalcite-editor;
        });

      apps = forAllSystems (system: {
        kalcite-editor = {
          type = "app";
          program = "${self.packages.${system}.kalcite-editor}/bin/kalcite-editor";
        };
        default = self.apps.${system}.kalcite-editor;
      });

      devShells = forAllSystems (system:
        let pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.mkShell {
            packages = [ pkgs.cargo pkgs.clippy pkgs.pkg-config pkgs.rustc pkgs.rustfmt ];
            buildInputs = with pkgs; [
              libGL libxkbcommon wayland xorg.libX11 xorg.libXcursor xorg.libXfixes
              xorg.libXi xorg.libXrandr xorg.libXrender xorg.libxcb
            ];
          };
        });
    };
}
