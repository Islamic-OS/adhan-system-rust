# adhan-system-rust

Our existing Adhan System, but Rust implementation.

## How to run

Make sure you have Rust (Cargo) and Git installed.

Then from your code directory:

```bash
> git clone https://github.com/Islamic-OS/adhan-system-rust.git
> cd adhan-system-rust
> cargo run
```

Dassit!

## Updates

### Routes Added

- `/` - For ping test and shows basic information.
- `/today` - For displaying the Prayer Times today.
- `/current` - For fetching the ongoing Prayer along with the time remaining, also for getting the next Prayer.
- `/qiblah` - For fetching the angle of the direction to Holy Ka'abaa from the users' coordinates (in terms of degrees as bearing).
