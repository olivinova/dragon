# Code Coverage Guide

This project uses `cargo-tarpaulin` to measure code coverage and `codecov` for tracking coverage over time.

## Local Coverage Reports

### Install tarpaulin

```bash
cargo install cargo-tarpaulin
```

### Generate coverage report

```bash
cd dragon-hoard

# HTML report
cargo tarpaulin --out Html --timeout 300

# Cobertura XML report (for CI)
cargo tarpaulin --out Xml --timeout 300

# Line coverage summary
cargo tarpaulin --out Stdout --timeout 300
```

### View HTML report

After running the HTML command, open `tarpaulin-report.html` in your browser to see:
- Per-file coverage percentages
- Line-by-line hit counts
- Functions and branches covered

## Coverage Thresholds

Current test coverage includes:

- **Economy (61% coverage)** — Looting, recruitment, training, vault, tick/production
- **Combat (58% coverage)** — Town conquest, dungeon exploration, military power
- **Management (65% coverage)** — Worker assignments, space designation, upkeep
- **Magic (70% coverage)** — Specializations, enchantments, research costs

## Continuous Integration

Code coverage is automatically generated on every push to `main` or `master` and uploaded to Codecov. Coverage badges and trend reports are available in the Codecov dashboard.

To maintain or improve coverage:
1. Write tests in the appropriate `tests/*.rs` file
2. Push to trigger CI, which runs tarpaulin automatically
3. Review coverage report on Codecov to identify gaps
4. Add tests for uncovered code paths

## Tarpaulin Configuration

Options used in CI:

- `--out Xml` — Generate Cobertura XML for Codecov integration
- `--timeout 300` — Set 5-minute timeout for long runs
- `--exclude-files tests/*` — Exclude test code from coverage metrics

For more options, run:

```bash
cargo tarpaulin --help
```
