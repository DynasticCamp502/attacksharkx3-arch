# Maintainer: DynasticCamp502 <https://github.com/DynasticCamp502>
# Contributor: Enthony Araujo <https://github.com/enthonyaraujo>

pkgname=attack-shark-x3
pkgver=1.0.0
pkgrel=1
pkgdesc="Native Linux configuration driver and GUI for the Attack Shark X3 gaming mouse"
arch=('x86_64')
url="https://github.com/enthonyaraujo/attacksharkx3-linux"
license=('MIT')
depends=('gcc-libs' 'glibc' 'hidapi' 'hicolor-icon-theme' 'libxkbcommon' 'wayland')
optdepends=(
    'libglvnd: OpenGL hardware acceleration support'
    'libx11: X11 display server support'
    'libxcursor: X11 cursor support'
    'libxi: X11 input support'
)
makedepends=('cargo')
source=(
    "$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/$pkgver.tar.gz"
    "fix-build.patch"
)
sha256sums=(
    '72a1b91c909fd841cc6cc46308d9b5eedff9140e73127315e83c45a4398e2110'
    '0f40a9518550b2a1c62cba606f50487183af5b2c49fbe65862e91b5daa3df81c'
)

prepare() {
    cd "attacksharkx3-linux-$pkgver"
    patch -Np1 -i "$srcdir/fix-build.patch"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
}

build() {
    cd "attacksharkx3-linux-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-targets
}

check() {
    cd "attacksharkx3-linux-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo test --frozen --all-targets
}

package() {
    cd "attacksharkx3-linux-$pkgver"

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
