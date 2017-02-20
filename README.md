# man-browsed #

A daemon that serves HTML-formatted man pages to your browser. Once configured,
can be invoked by simple typing "man TERM" into the address bar.

## Installation ##

Arch users can use the PKGBUILD in the 'dist' directory. Other users need to
build using Rust, and install the resulting single binary file and systemd
service.

## Usage ##

Start and enable the systemd service, then navigate to '127.0.0.1:53805' to see
the landing page. Follow the on-screen instructions to add the service as a new
search provider for the browser.
