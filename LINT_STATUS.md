# Linting Status

> Last checked: March 25, 2026

## Formatting

✅ **PASSED** - All Rust code has been formatted with `cargo fmt --all`

Note: Some rustfmt warnings appear due to nightly-only features in `rustfmt.toml`. These are expected and don't affect the formatting.

## Linting (Clippy)

⚠️ **162 WARNINGS** - Clippy found issues that should be addressed

### Summary of Issues

The clippy warnings fall into these categories:

1. **Documentation Issues** (~40%)
   - Missing `#[must_use]` attributes on methods
   - Missing `# Errors` sections in doc comments
   - Missing `# Panics` sections in doc comments

2. **Performance Issues** (~30%)
   - Inefficient pass-by-value vs pass-by-reference
   - Redundant closures that could use method references
   - Inefficient type casting (should use `From`/`Into`)

3. **Code Style Issues** (~20%)
   - Identical match arms that could be merged
   - Unnecessary `#[inline(always)]` attributes
   - Unused `self` parameters

4. **Type Safety Issues** (~10%)
   - Potential truncation in casts (usize → u32)
   - Unit type patterns that should use `()`
   - Borrow-as-pointer patterns

### Files with Most Issues

- `src/file_browser/fs/src/sorter.rs` - 15 warnings
- `src/file_browser/fs/src/splatter.rs` - 18 warnings
- `src/file_browser/fs/src/files.rs` - 8 warnings
- `src/file_browser/fs/src/provider/local/local.rs` - 9 warnings
- `src/file_browser/fs/src/provider/traits.rs` - 5 warnings

### Recommended Actions

#### For Contributors

When working on this codebase:

1. Run `cargo clippy --workspace --all-targets --all-features` before committing
2. Fix warnings in files you modify
3. Add appropriate documentation for public APIs
4. Use `#[allow(clippy::...)]` only when necessary with justification

#### For Maintainers

Priority fixes:

1. **High Priority**: Fix type safety issues (casts, truncation)
2. **Medium Priority**: Add missing documentation
3. **Low Priority**: Performance optimizations and style improvements

### Running Clippy

```bash
# Check for issues (warnings only)
cargo clippy --workspace --all-targets --all-features

# Auto-fix some issues
cargo clippy --fix --allow-dirty --allow-staged

# Check specific package
cargo clippy -p fb-fs --all-targets --all-features
```

### CI/CD Considerations

The current clippy configuration treats warnings as errors (`-D warnings`). This is good for maintaining code quality but may block builds.

Options:
1. Keep strict mode and fix all warnings
2. Allow warnings in CI but fail on errors only
3. Use `clippy.toml` to configure allowed lints

## Documentation Files

✅ **FORMATTED** - All markdown files are properly formatted:

- `AGENTS.md` - Codex CLI integration documentation
- `CODEX_INTEGRATION.md` - Detailed integration guide
- `QUICKSTART.md` - Quick setup guide
- `.github/CODEX_SETUP.md` - Contributor guide
- `.codex/AGENTS.override.md` - Critical project rules
- `.codex/config.toml` - Project-specific Codex config

## Configuration Files

✅ **VALID** - All TOML configuration files are valid:

- `.codex/config.toml` - Codex CLI configuration
- `Cargo.toml` - Workspace manifest
- `config.toml` - Application configuration

## Next Steps

1. Address high-priority clippy warnings (type safety)
2. Add missing documentation for public APIs
3. Consider creating a `clippy.toml` to configure allowed lints
4. Update CI/CD to handle clippy warnings appropriately

## Notes

- The file browser code (`src/file_browser/`) is based on yazi and inherits some of its patterns
- Some warnings may be intentional design decisions
- Review each warning individually before fixing
