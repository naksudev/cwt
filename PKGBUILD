pkgname=cwt
pkgver=1.0.0
pkgrel=1
pkgdesc="A tool to manage wallpaper themes using swww"
arch=('x86_64')
url="https://github.com/naksudev/cwt"
license=('MIT')
depends=('swww')
makedepends=('rust' 'cargo')
source=("git+$url#tag=v$pkgver")
sha256sums=('05747aeb23b0f5dd20cd2d1fd1ed1868b871661b026df695689a276a647f4f7a')

build() {
	cd "$srcdir/$pkgname-$pkgver"
	cargo build --release --locked
}

package() {
	cd "$srcdir/$pkgname-$pkgver/target/release"
	install -Dm755 cwt "$pkgdir/usr/bin/cwt"
}
