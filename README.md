# mcu-gen
**Generate safe, minimal HALs from SVD or PDF datasheets. In Rust and C.**

## Goals

 - Learn embedded development
 - Generate code for MCU HALs (automated)
 - Make a CLI that works from a SVD and datasheet

> This is a project primarely for learning

## Phase 0 – Define the Vision (Week 0)

Pick 1–2 target MCUs (for example STM32F103C8T6 and RP2040)
Choose Rust as the primary language and target environment (no_std)
Decide on project structure and naming conventions

## Phase 1 – SVD Parsing and Basic Metadata Extraction (Weeks 1–3)

Use the svd-parser Rust crate to load SVD files
Convert SVD data into structured JSON containing peripherals, registers, bitfields, base addresses, pins, and alternate functions
Implement a CLI command “mcu-gen parse --chip STM32F103C8T6” that outputs metadata.json

## Phase 2 – Generate a Minimal Rust HAL (Weeks 4–6)

Define trait-based Rust wrappers for GPIO, ADC, UART, etc.
Generate methods like PA0.output(), ADC.read(), UART.write()
Use the quote! and syn crates or a template engine to generate code files
Add a CLI command “mcu-gen generate --chip STM32F103C8T6 --lang rust” to emit src/gpio.rs, src/adc.rs, etc.

## Phase 3 – Runtime-Aware Capability Descriptions (Weeks 7–9)

Extend the metadata JSON to include runtime-accessible peripheral capabilities
Implement mcu.has_gpio(), mcu.has_adc() methods in mcu.rs
Ensure APIs fail gracefully (return Option or Result when a peripheral is unavailable)
Provide a simple example that queries and prints available peripherals

## Phase 4 – AI/NLP-Assisted Datasheet Parsing (Weeks 10–14)

Build a PDF parsing pipeline using an LLM (for example GPT) to extract register maps, peripheral summaries, and pin assignments
Convert extracted information into the same JSON format used for SVD data
Add a CLI command “mcu-gen parse-pdf <datasheet.pdf>” as a fallback when no SVD is available

## Phase 5 – Support Multiple Chips and Cross-Language Output (Weeks 15–20)

Add profiles for additional MCUs (for example RP2040, STM32G4, ESP32)
Implement C code generation (emit .h and .c files) alongside Rust output
Introduce feature flags so only used modules are compiled (to minimize code size)
Provide project scaffolding support (for example via cargo-generate for Rust and CMake for C)

## Phase 6 – Smart Assistant Mode (Weeks 21–25)

Create a “mcu doctor” CLI that analyzes existing code for peripheral conflicts or missing initializations
Add an interactive prompt: “Generate Rust main() for UART + ADC on STM32F103”
Have the assistant output complete code snippets, linker settings, and pin assignments

## Phase 7 – Documentation, Community, and Release (Week 26+)

Write comprehensive documentation (for example using mdBook or GitHub Pages)
Publish the Rust crate on crates.io and provide a C package for download
Create example projects for each supported MCU (STM32, RP2040, etc.)
Announce the project to the Rust Embedded Working Group, relevant forums, and social media








## Code structure (i don't include everything at once (it is gradual))

mcu-gen/
├── .github/
│   └── workflows/         # CI setup later (optional early)
├── crates/
│   ├── core/              # Core SVD parsing + metadata logic
│   ├── generator/         # Code generation logic
│   ├── cli/               # CLI binary (calls into core/generator)
├── examples/              # Mini projects per MCU
├── docs/                  # mdBook source or static site docs
├── metadata/              
|   ├── chip_name          # Where all the data about the chip is
|       ├── source.svd
|       ├── metadata.json  # Output JSON files per chip
|       ├── datasheet.pdf  # Datasheets to be parsed (for later phases)
├── Cargo.toml             # Top-level workspace file
├── README.md
└── LICENSE





## License

This project is licensed under either:

- Apache License, Version 2.0
- MIT license

at your option.
