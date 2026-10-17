# Maintainer: DynasticCamp502 <https://github.com/DynasticCamp502>
# Original Author: Enthony Araujo <https://github.com/enthonyaraujo/attacksharkx3-linux>

pkgname=attack-shark-x3
pkgver=1.0.0
pkgrel=1
pkgdesc="Native Linux configuration driver and GUI for the Attack Shark X3 gaming mouse"
arch=('x86_64')
url="https://github.com/DynasticCamp502/attacksharkx3-arch"
license=('MIT')
depends=('gcc-libs' 'glibc' 'hidapi' 'hicolor-icon-theme' 'libxkbcommon' 'wayland')
optdepends=(
    'libglvnd: OpenGL hardware acceleration support'
    'libx11: X11 display server support'
    'libxcursor: X11 cursor support'
    'libxi: X11 input support'
)
makedepends=('cargo' 'git')
source=("$pkgname::git+$url.git")
sha256sums=('SKIP')

_builddir() {
    if [ -d "$srcdir/$pkgname" ]; then
        echo "$srcdir/$pkgname"
    else
        echo "$startdir"
    fi
}

prepare() {
    cd "$(_builddir)"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "$(_builddir)"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-targets
}

check() {
    cd "$(_builddir)"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo test --frozen --all-targets
}

package() {
    cd "$(_builddir)"

    # Install binary executable
    install -Dm755 "target/release/attack-shark-x3" "$pkgdir/usr/bin/attack-shark-x3"

    # Install desktop entry
    install -Dm644 "packaging/desktop/attack-shark-x3.desktop" "$pkgdir/usr/share/applications/attack-shark-x3.desktop"

    # Install application icons
    install -Dm644 "assets/icons/icon.png" "$pkgdir/usr/share/icons/hicolor/256x256/apps/attack-shark-x3.png"
    install -Dm644 "assets/icons/icon.png" "$pkgdir/usr/share/icons/attack-shark-x3.png"
    install -Dm644 "assets/icons/icon.png" "$pkgdir/usr/share/pixmaps/attack-shark-x3.png"

    # Install udev hardware access rules
    install -Dm644 "packaging/udev/99-attack-shark-x3.rules" "$pkgdir/usr/lib/udev/rules.d/99-attack-shark-x3.rules"

    # Install license documentation
    install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
