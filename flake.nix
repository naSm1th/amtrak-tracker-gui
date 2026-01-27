{
  description = "Rust flake";
  inputs =
    {
      nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05"; # or whatever vers
    };
  
  outputs = { self, nixpkgs, ... }@inputs:
    let
     system = "x86_64-linux"; # your version
     pkgs = nixpkgs.legacyPackages.${system};    
    in
    {
      devShells.${system}.default = pkgs.mkShell
      {
        packages = with pkgs; [ 
            rustup
            nodejs_24
            pnpm
            pkg-config
            glib
            curl
            wget
            dbus
            openssl_3
            gtk3
            libsoup_2_4
            webkitgtk_4_1
            librsvg
            bash
            llvmPackages.bintools
            protobuf
       ]; # whatever you need
      };
    };
}
