# Release Verification Guide

This document provides steps to verify a successful release of the ChainCraft Rust project.

## GitHub Actions Workflow Verification

1. After pushing a tag (e.g., `git push origin v0.1.0`), visit the GitHub Actions tab:
   https://github.com/jio-gl/chaincraft-rust/actions

2. You should see the release workflow triggered by the tag push
   - Verify that all jobs in the workflow complete successfully
   - Look for any failed steps and their error messages
   - If there are failures, check the workflow logs for details

3. The release workflow should include these successful steps:
   - Running tests
   - Verifying version matches between tag and Cargo.toml
   - Building documentation
   - Publishing to crates.io (if CARGO_REGISTRY_TOKEN is set)
   - Building binaries for multiple platforms
   - Creating a GitHub release with assets

## GitHub Release Verification

1. Check the GitHub repository releases page:
   https://github.com/jio-gl/chaincraft-rust/releases

2. Verify that the release includes:
   - The version tag (e.g., `v0.1.0`)
   - The changelog as the description
   - Binary assets for all platforms:
     - `chaincraft-rust-linux.tar.gz`
     - `chaincraft-rust-macos.tar.gz`
     - `chaincraft-rust-windows.zip`

3. Download and test at least one of the binaries to ensure it works properly

## Crates.io Verification

1. Check that the package is available on crates.io:
   https://crates.io/crates/chaincraft-rust

2. Verify that the following information is correct:
   - Version number
   - Description
   - Repository link
   - License
   - README content

3. Test installation from crates.io:
   ```bash
   cargo install chaincraft-rust
   ```

4. Run a basic command to verify it works:
   ```bash
   chaincraft-rust --version
   ```

## Documentation Verification

1. Check that the documentation is available on docs.rs:
   https://docs.rs/chaincraft-rust

2. Verify that all modules are properly documented and accessible

3. If GitHub Pages is set up, check the documentation there:
   https://jio-gl.github.io/chaincraft-rust/

## CI/CD Pipeline Verification

1. Check the GitHub Actions workflows for the release tag:
   https://github.com/jio-gl/chaincraft-rust/actions

2. Verify that all workflows completed successfully:
   - Build and test workflow
   - Release workflow
   - Documentation deployment

## If the Release Failed

If the release process fails, follow these steps to diagnose and fix:

1. Check the workflow logs for specific error messages
2. Common issues include:
   - Missing CARGO_REGISTRY_TOKEN secret
   - Version mismatch between tag and Cargo.toml
   - Test failures
   - Build failures for specific platforms

3. After fixing the issue:
   - If needed, delete the failed GitHub release
   - Delete the tag locally and remotely:
     ```bash
     git tag -d v0.1.0
     git push --delete origin v0.1.0
     ```
   - Fix the issue, then create and push the tag again

## Post-Release Steps

1. Announce the release through appropriate channels:
   - Project website or blog (if applicable)
   - Social media
   - Relevant forums or communities

2. Plan for the next development cycle:
   - Update the version in `Cargo.toml` to the next development version
   - Create new milestone for the next release
   - Prioritize issues and features for the next release

## Troubleshooting

If any verification steps fail:

1. Check the GitHub Actions logs for errors
2. Verify that all required secrets are set up correctly
3. For crates.io issues, ensure the `CARGO_REGISTRY_TOKEN` is valid and has publishing rights
4. For documentation issues, check the docs.rs build logs 