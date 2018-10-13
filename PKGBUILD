# Maintainer: Jake Howard <git@theorangeone.net>
pkgname=rofi-todoist
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    return 0
}

package() {
    cd $srcdir

    cargo build

    install -D -m755 "../target/release/rofi-todoist" "$pkgdir/usr/bin/rofi-todoist"
}
