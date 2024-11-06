# RustyNES

**RustyNES** is a NES (Nintendo Entertainment System) emulator written in Rust, designed to emulate the classic 8-bit gaming experience. This project aims to accurately recreate the behavior of the NES’s 6502 CPU, PPU (Picture Processing Unit), and memory architecture, making it a great learning resource for low-level programming, emulation techniques, and Rust development.

## Features
- **6502 CPU Emulation**: Full support for the NES’s 8-bit CPU, including all opcodes and addressing modes.
- **Memory Management**: Accurate memory mapping to mimic NES’s hardware.
- **Graphics Rendering**: *(Planned)* Emulation of the NES PPU for displaying graphics.
- **Controller Input**: *(Planned)* Emulation of NES controller input for game interactivity.

## Getting Started
To get started with RustyNES, clone the repository and build the project:

```bash
git clone https://github.com/yourusername/RustyNES.git
cd RustyNES
cargo build
```

### Running the Emulator
Once built, you can run the emulator with:

```bash
cargo run
```

## Project Goals
- **Learn and implement NES hardware components**: Focus on accurately simulating the NES’s 6502 CPU, PPU, and APU (Audio Processing Unit).
- **Develop in Rust**: Explore Rust’s performance and safety features in low-level emulation.
- **Build a foundation for classic NES game support**: Aim to support popular NES titles as development progresses.

## Contributing
Contributions are welcome! Feel free to fork the project, submit issues, or create pull requests to help improve RustyNES.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.