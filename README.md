# coverage-badge

![Coverage](assets/coverage.svg)

A GitHub Action and CLI tool that generates shields.io-style SVG badges for test coverage.

## Quick Start (GitHub Action)

Add this step to your workflow after running your coverage tool:

```yaml
- name: Generate coverage badge
  uses: ozankasikci/rust-test-coverage-badge@v1
  with:
    coverage: ${{ steps.coverage.outputs.percentage }}
    output: assets/coverage.svg
```

Then reference the badge in your README:

```markdown
![Coverage](assets/coverage.svg)
```

## Full Workflow Example

```yaml
name: Coverage

on:
  push:
    branches: [main]

permissions:
  contents: write

jobs:
  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      # Replace with your actual coverage tool
      - name: Run tests with coverage
        id: coverage
        run: |
          # Example: extract coverage from your tool's output
          # For tarpaulin: jq '.coverage' tarpaulin-report.json
          # For pytest-cov: grep -oP 'TOTAL.*\s+\K\d+' coverage.txt
          echo "percentage=85.5" >> $GITHUB_OUTPUT

      - name: Generate coverage badge
        uses: ozankasikci/rust-test-coverage-badge@v1
        with:
          coverage: ${{ steps.coverage.outputs.percentage }}
          output: assets/coverage.svg

      # Optional: commit the updated badge
      - name: Commit badge
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add assets/coverage.svg
          git diff --staged --quiet || git commit -m "Update coverage badge"
          git push
```

## Action Inputs

| Input | Required | Description |
|-------|----------|-------------|
| `coverage` | Yes | Coverage percentage (0-100, decimals allowed) |
| `output` | Yes | Output path for the SVG file |

## CLI Usage

For local development or custom setups, you can use the CLI directly.

### Install via Cargo

```bash
cargo install coverage-badge
```

### Install from Releases

Download the binary for your platform from [Releases](https://github.com/ozankasikci/rust-test-coverage-badge/releases).

### Run

```bash
coverage-badge --coverage 85 --output assets/coverage.svg
```

## Color Thresholds

| Coverage | Color |
|----------|-------|
| < 50% | Red |
| 50-79% | Yellow |
| ≥ 80% | Green |

## License

MIT
