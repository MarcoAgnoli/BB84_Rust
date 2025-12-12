

# BB84 – Concurrent Version (Rust)

Concurrent simulation of the BB84 quantum key distribution protocol.

## Features
- Concurrency with separate threads for **Writer**, **Reader**, and optional **Attacker**
- Synchronization via flags on **Public/Quantum Channel** (lightweight waiting)
- Output includes:
  - Photon sequences (Writer / Attacker if active / Reader)
  - Final symmetric keys for Writer and Reader
  - Statistics on lost photons

## Requirements
- Rust (stable)
- Visual Studio Code with **Rust Analyzer** extension

## How to Run
1. Open the project folder in **VS Code**
2. Go to **Run Task** → `BB84: Run`
3. The task executes `cargo run` and displays output in a dedicated panel

## Configuration
Edit `main.rs` to adjust parameters:
- `LUNG_MSG = 64` (8 bytes, 64 photons)
- `ATTIVA_AVVERSARIO = false` (set to `true` to enable attacker)

## Notes
- The Reader waits for `letto_da_avversario = true` (if attacker is active) before reading each photon, ensuring the order Attacker → Reader.
- Sequences and keys are stored in debug fields of the Public Channel for printing in `main`.
- Code generated with assistance from Microsoft Copilot based on a detailed specification.

---

### License
MIT License
