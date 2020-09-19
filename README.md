
# libimobiledevice-rs

`libimobiledevice-rs` provides Rust bindings for the [libimobiledevice](https://github.com/libimobiledevice/libimobiledevice), [libirecovery](https://github.com/libimobiledevice/libirecovery), and [libusbmuxd](https://github.com/libimobiledevice/libusbmuxd) libraries, for interfacing with iOS devices.

## Documentation

[libimobiledevice-sys](https://aspenuwu.me/docs/libimobiledevice-rs/libimobiledevice_sys/)  
[libirecovery-sys](https://aspenuwu.me/docs/libimobiledevice-rs/libirecovery_sys/)  
[libusbmuxd](https://aspenuwu.me/docs/libimobiledevice-rs/libusbmuxd_sys/)  

## Licenses

### libimobiledevice/libusbmuxd/libirecovery

[libimobiledevice, libusbmuxd, and libirecovery are licensed under the LGPL 2.1 license.](https://www.gnu.org/licenses/old-licenses/lgpl-2.1.en.html)

As libimobiledevice-rs dynamically links to these, and only uses the headers on the system, we are
not contaminated by the LGPL, and can therefore license under the Zlib license.

### libimobiledevice-rs

The zlib/libpng License

Copyright (c) `2020` `aspen`

This software is provided 'as-is', without any express or implied warranty. In
no event will the authors be held liable for any damages arising from the use of
this software.

Permission is granted to anyone to use this software for any purpose, including
commercial applications, and to alter it and redistribute it freely, subject to
the following restrictions:

1.  The origin of this software must not be misrepresented; you must not claim
    that you wrote the original software. If you use this software in a product,
    an acknowledgment in the product documentation would be appreciated but is
    not required.

2.  Altered source versions must be plainly marked as such, and must not be
    misrepresented as being the original software.

3.  This notice may not be removed or altered from any source distribution.
