# MKDedede

A Rust crate for decoding and encoding Mario Kart: Double Dash!! and Mario Kart DS ghost data passwords

---

The games generate a 16-character password after a time trial that encodes some info about them. This crate parses and validates those passwords.

Double Dash ported from the JavaScript decoder by [WaluigiBSOD](https://github.com/WaluigiBSOD/mkdd-password-decoder).

DS ported from the C decoder by [simontime](https://github.com/simontime/MKDSTTCodeDecoder/blob/master/decoder.c).

## Simple Double Dash decoding example

```rust
use mkdedede::dd::decode;

match decode("SOME16CHARPASSWD") {
    Ok(ghost) => println!("{}", ghost),
    Err(e)    => eprintln!("invalid password: {}", e),
}
```

## License

AGPL-3.0, see [LICENSE](LICENSE)
