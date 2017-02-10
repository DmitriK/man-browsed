# Maintainer: Dmitri Kourennyi <dkour@mykolab.com>
pkgname=man-browsed-git
pkgver=0.1.0
pkgrel=1
pkgdesc="Daemon for serving man pages over HTML"
arch=("any")
url="https://github.com/DmitriK/man-browsed"
license=('unkown')
groups=()
depends=('systemd')
makedepends=('git' 'rust')
# source=()
# md5sums=() #generate with 'makepkg -g'

_gitroot=https://github.com/DmitriK/man-browsed.git
_gitname=man-browsed

build() {
  cd "$srcdir"
  msg "Connecting to GIT server...."

  if [[ -d "$_gitname" ]]; then
    cd "$_gitname" && git pull origin
    msg "The local files are updated."
  else
    git clone "$_gitroot" "$_gitname"
  fi

  msg "GIT checkout done or server timeout"
  msg "Starting build..."

  rm -rf "$srcdir/$_gitname-build"
  git clone "$srcdir/$_gitname" "$srcdir/$_gitname-build"
  cd "$srcdir/$_gitname-build"

  cargo build --release
}

package() {
  cd "$srcdir/$_gitname-build"

  install -dm 755 "${pkgdir}"/{usr/bin,usr/lib/systemd/system}
  install -m 755 ./target/release/man-browsed "${pkgdir}"/usr/bin/man-browsed
  install -m 644 ./man-browsed.service "${pkgdir}"/usr/lib/systemd/system/
}

# vim:set ts=2 sw=2 et:
