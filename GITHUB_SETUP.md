# GitHub Repository Setup Guide

This document outlines the steps needed to properly configure the GitHub repository for the ChainCraft Rust project.

## Current Workflow Files

The repository contains two GitHub Actions workflow files:

1. **ci.yml** - Continuous Integration workflow that runs tests, linting, and other checks on each push and pull request.
2. **release.yml** - Release workflow that is triggered by tag pushes (v*) to build and publish releases.

## Branch Protection Rules

Set up branch protection for the `main` branch:

1. Go to the repository settings: https://github.com/jio-gl/chaincraft-rust/settings/branches
2. Under "Branch protection rules", click "Add rule"
3. For "Branch name pattern", enter `main`
4. Enable the following settings:
   - ✅ Require pull request reviews before merging
   - ✅ Require status checks to pass before merging
     - Required status checks: `test`, `docs`, `clippy`
   - ✅ Require branches to be up to date before merging
   - ✅ Include administrators
5. Click "Create" to save the rule

## Repository Secrets

Add the following secrets for the CI/CD workflows:

1. Go to Settings > Secrets and variables > Actions
2. Add the following secrets:

### CARGO_REGISTRY_TOKEN
This token is used to publish to crates.io:
1. Generate a token at https://crates.io/settings/tokens
2. Select "Publish" scope
3. Add the token as a secret named `CARGO_REGISTRY_TOKEN`

### CODECOV_TOKEN (Optional)
If using Codecov for code coverage:
1. Set up a project on https://codecov.io/
2. Get the repository token
3. Add the token as a secret named `CODECOV_TOKEN`

## GitHub Pages Setup

To enable GitHub Pages for documentation:

1. Go to Settings > Pages
2. Under "Source", select "GitHub Actions" 
3. This allows the CI pipeline to deploy docs through the workflow

## Verifying Workflows

To verify that your workflows are properly configured and running:

1. Check the GitHub Actions tab: https://github.com/jio-gl/chaincraft-rust/actions
2. You should see the CI workflow running on each push to the repository
3. When you push a tag (e.g., `v0.1.0`), the release workflow should be triggered automatically
4. If workflows aren't running, check that:
   - The workflow files are in the correct location (.github/workflows/)
   - The workflow syntax is valid
   - The repository has appropriate permissions to run workflows

## Future Organization Transfer

When the `chaincraft-org` organization is created:

1. Create the organization on GitHub
2. Go to Settings > General > Transfer ownership
3. Enter the organization name `chaincraft-org`
4. Follow the prompts to complete the transfer
5. After transfer, update `Cargo.toml` repository URLs
6. Re-configure branch protection and secrets in the new repository

## Post-Transfer Verification

After transferring the repository:

1. Verify CI/CD workflows still run properly
2. Update any external links to the repository
3. Ensure team members have proper access to the new repository 