# TinyShark

A lightweight packet capture tool written in Rust. Inspired by Wireshark, but focused on simplicity and minimal setup.

##  Current State (MVP in progress)

Rust project initialized with modular layout.

### Dependencies:

- pcap
 — capture packets via Npcap/WinPcap.

- clap
 — CLI argument parsing.

- anyhow
 — ergonomic error handling.

- serde, serde_json — saving summaries.

## Build setup:

Npcap runtime installed (WinPcap API-compatible mode).

SDK .lib vendored into vendor/npcap/lib. - x64


## Project Layout
src/
  - main.rs        // entrypoint
  - capture.rs     // start/stop capture, packet loop, summary
  - list.rs        // list available interfaces
  - files.rs       // helper for output directory + filenames
  - tally.rs       // aggregate stats

vendor/
  - npcap/         // minimal Npcap SDK (wpcap.lib, includes)

## MVP Goals
Commands

list → enumerate available interfaces.
rec -i <index> → capture on chosen interface until stopped:
Save raw packets to .pcap in Documents/TinySharkCaptures.
Save summary (summary.json) with:
Total packets
Total bytes
Duration
Top flows (proto, src:port → dst:port)
Top talkers (per-IP traffic).

read -r <file.pcap> → reprocess saved capture offline to regenerate summary.

# How to Setup:
- 1:  clone the repo using "git clone https://github.com/josephdoba/tinyshark.git"

- 2: `cargo build` then `cargo run` in the root directory

## Next Steps

Implement list_interfaces() in list.rs.

Expand capture() to loop packets, count totals, return a Summary.

Add graceful stop condition (Ctrl+C).

Implement file helpers (files.rs) → auto-create Documents/TinySharkCaptures.

Add summary.json output (via serde_json).

Wire up CLI with clap (subcommands: list, rec, read).

## Notes

Requires Npcap runtime on Windows (checked during install).

Only capture traffic you own or have explicit permission to analyze.