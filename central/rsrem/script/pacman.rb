# frozen_string_literal: true

def list_installed(pkg)
  pkgs = system('pacman -Q')

  out = /#{pkg}/.match(pkgs).captures
  puts String(out)
end

list_installed('gcc')
