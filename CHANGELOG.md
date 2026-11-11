## [Beta 6.5] - 2026-11-11

### Fixed
- Password hashing upgraded from iterated SHA-256 to PBKDF2-HMAC-SHA256 (100,000 iterations) with a per-profile random salt.
- Removed hard-coded cryptographic values from tests (dynamic, randomly generated test data instead).

## [Beta 7] - 2026-11-11

### Added
- Drag & drop files into the message bar to attach them (images and text/code files).
- Clear warning shown in the composer when a file can't be attached, explaining why.

### Changed
- Attachment flow now matches the `+` picker (768px image resize, 8 MB limit).

## [Beta 6] - 2026-11-11

### Added
- Full source code published to the `main` branch.
- Refreshed `SECURITY.md` with improved formatting and expanded context.
- macOS `.app` bundle with icon for eAI Lite.

### Changed
- Rebuilt macOS and Windows binaries from source.

### Fixed
- Eliminated dead-code warnings in `archive.rs` and `net.rs`.

PR: https://github.com/JustEroDeveloper/eAI/pull/6

# Changelog

All notable changes to **eAI Lite** are documented here.

## [Beta 5] - 2026-11-10

### Added
- Attach `.txt` and code files through the `+` icon in the message bar.
- Removable "file attached" chips in the composer and message bubbles.
- Censorless mode toggle in **Settings → General**.
- External API provider support in **Settings → API** (Ollama or OpenAI-compatible).
- macOS app bundle (`.dmg` / `.zip`) alongside Windows `.exe`.
- Broader Unicode font fallbacks to reduce empty-box/tofu glyphs.

### Improved
- UI overhaul: refreshed dark palette, larger rounding, cleaner cards, input bar, and message bubbles.
- Thought process header now uses plain ASCII arrows instead of missing glyphs.
- Send button enables when a file is attached even without text.

### Fixed
- Empty-box rendering for unsupported Unicode characters in generated text and thinking output.
- File contents are now included in the prompt sent to the model.

## [Beta 4] - 2026-11-10

### Added
- Local profiles and sign-in flow.
- Per-profile settings and conversations.

## [Beta 3] - 2026-11-10

### Added
- Web search integration and source citations.
- Thinking/reasoning view for supported models.

## [Beta 2] - 2026-11-09

### Added
- Model renaming and settings panels.
- Image attachments.

## [Beta 1] - 2026-11-09

### Added
- Initial Windows release of eAI Lite.