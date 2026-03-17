# MKDedede

A Rust crate for decoding and encoding Mario Kart: Double Dash!! and Mario Kart DS ghost data passwords

---

The games generate a 16-character password after a time trial that encodes some info about them. This crate parses and validates those passwords.

Double Dash ported from the JavaScript decoder by [WaluigiBSOD](https://github.com/WaluigiBSOD/mkdd-password-decoder).

DS ported from the C decoder by [simontime](https://github.com/simontime/MKDSTTCodeDecoder/blob/master/decoder.c).

## Simple Double Dash decoding example

```rust
use mkdedede::double_dash::decode;

match decode("SOME16CHARPASSWD") {
    Ok(ghost) => println!("{}", ghost),
    Err(e)    => eprintln!("invalid password: {}", e),
}
```

## Usage from C/C++ (FFI)

We provide an FFI layer that exposes C-compatible functions to decode and encode passwords!

To build for C/C++, use cargo to build the native bindings:

```bash
cargo build --release
```

This will produce `.so`, `.dylib`, or `.dll` (and their respective static architectures) inside `target/release/`.

Include the provided header from `include/mkdedede.h` in your project and link the output shared library.

```c
#include <stdio.h>
#include "mkdedede.h"

int main() {
    MkddGhostDataC data;
    MkdededeDecodeStatus status = mkdedede_mkdd_decode("NJTXAJFLY4M7BGGG", &data);

    if (status == MKDEDEDE_SUCCESS) {
        printf("Course: %s\n", mkdedede_mkdd_course_name(data.course));
        printf("Kart: %s\n", mkdedede_mkdd_kart_name(data.kart));

        char time_str[16] = {0};
        mkdedede_mkdd_format_time(data.total_time_ms, time_str);
        printf("Total Time: %s\n", time_str);
    } else {
        printf("Decode failed with status: %d\n", status);
    }
    return 0;
}
```

## License

AGPL-3.0, see [LICENSE](LICENSE)
