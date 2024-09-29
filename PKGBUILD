pkgname=cwt
pkgver=1.0.0
pkgrel=1
pkgdesc="A tool to manage wallpaper themes using swww"
arch=('x86_64')
url="https://github.com/naksudev/cwt"
license=('MIT')
depends=('swww' 'git')
makedepends=('rust' 'cargo')
source=("git+$url#tag=$pkgver")
sha256sums=('SKIP')

build() {
	cd "$srcdir/$pkgname"
	cargo build --release --locked
}

package() {
	cd "$srcdir/$pkgname/target/release"
	install -Dm755 cwt "$pkgdir/usr/bin/cwt"
}
