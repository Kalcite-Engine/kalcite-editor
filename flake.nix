{
  description = "Kalcite Editor";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: { packages.x86_64-linux.kalcite-editor =
    (import nixpkgs { system = "x86_64-linux"; }).rustPlatform.buildRustPackage {
      pname = "kalcite-editor"; version = "0.14.0"; src = ./.;
      cargoLock = { lockFile = ./Cargo.lock; allowBuiltinFetchGit = true; };
    }; };
}
