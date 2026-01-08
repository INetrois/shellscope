pkgname=shellscope
pkgver=0.1.0
pkgrel=1
pkgdesc="Lightweight CLI tool for structured command execution"
arch=('x86_64')
url="https://github.com/INetrois/shellscope"
license=('Apache-2.0')
depends=('gcc-libs')
makedepends=('cargo' 'git')
source=("$pkgname::git+https://github.com/INetrois/shellscope.git")
sha256sums=('SKIP')

build() {
    cd "$pkgname"
    cargo build --release --locked
}

package() {
    cd "$pkgname"

    install -Dm755 target/release/shellscope \
        "$pkgdir/usr/bin/shellscope"

    install -Dm644 LICENSE \
        "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
