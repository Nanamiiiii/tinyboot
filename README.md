# tinyboot

tiny secure bootloader project for embedded system written in Rust  
Assumed to be used for **research purposes**  
Now targeting only RISC-V processor

## ✨ Features

Some are in the conceptual stage now.

- Multi architecture
- Simple SecureBoot Implementation
- Minimal hardware abstraction layer / driver
- Small footprint (ideally)
- `cargo-make` based Kconfig like build-system
- High extensibility

## 🎨 Project Structure

This project consists of the following components

- libtinyboot
  - core library of the tinyboot security logic
- loader
  - main source of the bootloader
  - separated by each target
- arch
  - architecture specific components (e.g. asm source)
- board
  - target board specific components (e.g. hal, drivers)

## 🛠️ Building

### Pre-requisites

- Rust (>= 1.60)
  - Must install required target (e.g. `riscv64imac-unknown-none-elf`)
- Cargo
- `cargo-make`
  - For details: https://github.com/sagiegurari/cargo-make

### Build bootloader

This is the example of building for RISC-V QEMU virt target (riscv64)

```shell
# Configure build target
cargo make defconfig virt-riscv64
# Build
cargo make build
```

In this example, two artifacts are generated in `loader/virt-riscv64`

- `tinyboot-fw.bin`
  - firmware loader
  - same functionality as FSBL, U-Boot SPL
- `tinyboot.bin`
  - OS loader
  - same functionality as U-Boot Proper

### Build system

🚧 WIP

### Adding new target

🚧 WIP

## 🗺️ Roadmap

- [x] Project structure
- [x] Create prototype build-system
- [ ] Emulation target (`virt-riscv64`)
  - [ ] Implement minimum hal / SBI interface
  - [ ] Implement simple loader
  - [ ] Implement core security logic
- [ ] Porting to real hardware target (HiFive Unmatched or VisionFive ...?)
- [ ] ...

## ✏️ Author

- Akihiro Saiki - Main worker - [Nanamiiiii](https://github.com/Nanamiiiii)

## ©️ License

This project is licensed under the Apache 2.0 License - see the [LICENSE](./LICENSE) for details
