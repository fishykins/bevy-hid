# Bevy_hid
<div id="top"></div>

[![Minimum Supported Rust Version]][Rust 1.56]

Support for the Windows [HID device protocol](https://docs.microsoft.com/en-us/windows-hardware/drivers/hid/) in [Bevy](https://github.com/bevyengine/bevy).

<!-- ABOUT THE PROJECT -->
## About The Project

Bevy_hid is an alternative to the the default [bevy-gilrs](https://github.com/bevyengine/bevy/tree/main/crates/bevy_gilrs) crate, implimenting the [Human Interface Device](https://docs.microsoft.com/en-us/windows-hardware/drivers/hid/) Windows protocol instead.
This is a more involved approach to peripheral connectivity and allows for the use of less standard devices as game input, as well as providing a simple yet powerful mapping system for unknown devices. 
Because this approach takes advantage of the base bevy input system, it can seemlessly integrated into other input libraries (e.g. [LIM](https://crates.io/crates/leafwing-input-manager)) with no extra code required.

## Limitations

I have not tested or put much thought into handling multiple devices of the same type, which may be problematic if you want to rock
a dual joystick configuration. If you are this way inclined, get in touch and I can look into implimentation.

<!-- GETTING STARTED -->
## Getting Started

Clone me! I will be on crates.io soon...

### Optional features


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are both welcome and appreciated!

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's [Code of Conduct].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

If you are a device vendor and you want your gear to be natively supported, please get in touch. 
If you are not a device vendor but you want to send me a device for testing, also get in touch. 
If you are an AC-10 warthog who wants to contribute air support, absolutely get in touch.

<!-- CONTACT -->
## Contact

Caspar Green - caspar.m.green@gmail.com

Project Link: [https://github.com/fishykins/sticky-joy](https://github.com/fishykins/sticky-joy)

<p align="right">(<a href="#top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[Latest Version]: https://img.shields.io/crates/v/sticky-joy.svg
[crates.io]: https://crates.io/crates/sticky-joy/
[Minimum Supported Rust Version]: https://img.shields.io/badge/Rust-1.56.0-blue?color=fc8d62&logo=rust
[Rust 1.56]: https://github.com/rust-lang/rust/blob/master/RELEASES.md#version-1560-2021-10-21
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[contributors]: https://github.com/fishykins/sticky-joy/graphs/contributors
[docs]: "https://img.shields.io/docsrs/sticky-joy/"
[docs.rs]: "https://docs.rs/sticky-joy/latest/sticky-joy/"