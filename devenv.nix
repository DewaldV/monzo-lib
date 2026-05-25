{ pkgs, lib, ... }:

{
  languages.rust = {
    enable = true;
    channel = "stable";
    mold.enable = true;
  };

  packages = with pkgs; [
    pkg-config
    clang
    openssl
  ];
}
