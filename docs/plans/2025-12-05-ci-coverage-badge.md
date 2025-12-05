# CI Coverage Badge Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Set up GitHub Actions to run tests with coverage, generate a badge using our tool, and commit it back to the repo.

**Architecture:** GitHub Actions workflow runs cargo-tarpaulin, extracts coverage percentage, runs coverage-badge to generate SVG, commits if changed.

**Tech Stack:** GitHub Actions, cargo-tarpaulin, coverage-badge (our tool)

---

## Task 1: Create GitHub Actions Workflow

**Files:**
- Create: `.github/workflows/coverage.yml`

**Step 1: Create the workflow file**

Create `.github/workflows/coverage.yml`:
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
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable

      - name: Install cargo-tarpaulin
        run: cargo install cargo-tarpaulin

      - name: Run tests with coverage
        run: cargo tarpaulin --out json --output-dir target/coverage

      - name: Extract coverage percentage
        id: coverage
        run: |
          COVERAGE=$(cat target/coverage/tarpaulin-report.json | jq '.coverage_percentage')
          echo "percentage=$COVERAGE" >> $GITHUB_OUTPUT
          echo "Coverage: $COVERAGE%"

      - name: Build coverage-badge
        run: cargo build --release

      - name: Generate badge
        run: ./target/release/coverage-badge -c ${{ steps.coverage.outputs.percentage }} -o assets/coverage.svg

      - name: Commit badge
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "github-actions[bot]@users.noreply.github.com"
          git add assets/coverage.svg
          git diff --staged --quiet || git commit -m "Update coverage badge"
          git push
```

**Step 2: Verify the file structure**

Run: `ls -la .github/workflows/`
Expected: `coverage.yml` exists

**Step 3: Commit**

```bash
git add .github/workflows/coverage.yml
git commit -m "Add GitHub Actions workflow for coverage badge"
```

---

## Task 2: Update README with Badge

**Files:**
- Modify: `README.md`

**Step 1: Add badge to README**

Update the top of `README.md` to add the badge after the title:
```markdown
# coverage-badge

![Coverage](assets/coverage.svg)

A Rust CLI tool that generates shields.io-style SVG badges for test coverage.
```

**Step 2: Commit**

```bash
git add README.md
git commit -m "Add coverage badge to README"
```

---

## Task 3: Test Workflow Locally (Verification)

**Step 1: Verify current tests pass**

Run: `cargo test`
Expected: All 15 tests pass

**Step 2: Verify tarpaulin output format (manual check)**

Note: tarpaulin requires Linux. On macOS, we verify the workflow syntax is correct.

Run: `cat .github/workflows/coverage.yml`
Expected: Valid YAML with all required steps

**Step 3: Verify badge generation still works**

Run: `cargo run --release -- -c 85 -o /tmp/test-badge.svg && cat /tmp/test-badge.svg | head -5`
Expected: Valid SVG output
