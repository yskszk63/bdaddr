[![Crates.io](https://img.shields.io/crates/v/bdaddr)](https://crates.io/crates/bdaddr)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/yskszk63/bdaddr/CI)](https://github.com/yskszk63/bdaddr/actions/workflows/ci.yml)
[![Libraries.io dependency status for GitHub repo](https://img.shields.io/librariesio/github/yskszk63/bdaddr)](https://libraries.io/cargo/bdaddr)
![Codecov](https://img.shields.io/codecov/c/gh/yskszk63/bdaddr)

# bdaddr

Bluetooth Device Address.

### Dependencies

```toml
[dependencies]
bdaddr = "0.1"
```

### Example

```rust
use bdaddr::{Address, RandomDeviceAddress, StaticDeviceAddress};

fn main() {
    let addr = Address::le_random_from_str("00:11:22:33:44:55").unwrap();
    assert_eq!(addr.to_string(), "00:11:22:33:44:55");
    assert!(matches!(addr, Address::LeRandom(RandomDeviceAddress::NonResolvable(..))));
}
```

### License

Licensed under either of
* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.!
