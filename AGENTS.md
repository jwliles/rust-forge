# AGENTS.md

## Purpose

These instructions define how coding agents should inspect, search, edit, and validate this repository.

Prefer existing project conventions over generic defaults. Make the smallest correct change needed for the task.

## Search and Navigation

Use the appropriate tool for the kind of search being performed.

### File discovery

Use `fd` for locating files and directories.

### Text search

Use `rg` for literal, regex, comment, string, configuration, documentation, and other text-oriented searches.

Do not use regex to approximate a syntax-aware search when `ast-grep` can express the query structurally.

### Structural code search

Use `ast-grep` for syntax-aware searches involving:

- function calls
- method calls
- declarations
- expressions
- assignments
- control flow
- nested syntax
- structurally equivalent code with different formatting

Prefer the normal human-readable output when showing results.

Examples:

```bash
ast-grep -p '$A.unwrap()' -l rust
ast-grep -p 'DateTime::parse_from_rfc3339($A)' -l rust
ast-grep -p 'fetch($ARG)' -l ts
ast-grep -p 'open($ARG)' -l python
```

Use metavariables such as `$A`, `$B`, `$ARG`, and `$$$ARGS` where appropriate.

Use JSON only when programmatic processing is useful:

```bash
ast-grep -p '$A.unwrap()' -l rust --json=stream
```

Before performing a structural refactor, search for all structurally equivalent occurrences first.

For structural rewrites, prefer an interactive review before applying changes broadly:

```bash
ast-grep -p '<pattern>' -r '<replacement>' -l <language> -i
```

Do not use `-U` / `--update-all` until the transformation has been verified.

## Tool Roles

Treat the local tool as separate layers:

```text
ast-grep -> structural and syntax-aware code search
```

Do not collapse structural search into RG-style regex unless syntax-aware search is unavailable or inappropriate.

## Editing

- Read enough surrounding code to understand the local conventions before editing.
- Preserve established naming, formatting, architecture, and error-handling patterns.
- Avoid unrelated cleanup.
- Do not reformat entire files unless required by the project's formatter or explicitly requested.
- Do not modify generated files unless the task specifically requires it.
- Do not modify files when the request is only for analysis, explanation, or search results.
- For scripts intended to be executable or used as system modules, include an appropriate shebang.

## Language-Specific Preferences

### Rust

- Prefer idiomatic Rust and explicit error propagation over unnecessary `unwrap()` or `expect()`.
- Use `cargo fmt` for formatting when appropriate.
- Use `cargo check` or the project's existing validation command before broader test runs when practical.
- Respect existing Clippy configuration and project lint settings.

### Python

- Follow the repository's existing style and type-hint conventions.
- Prefer `ruff` for formatting and linting when the project is configured for it.
- Prefer `uv` for environment or dependency workflows when already used by the project.
- Include a shebang on executable scripts.

### TypeScript / JavaScript

- Follow the project's existing package manager and `packageManager` declaration.
- Prefer the configured formatter/linter, including Biome when present.
- Do not replace project tooling with global defaults.
- Preserve existing module-system conventions.

## Validation

After making changes:

1. Run the narrowest relevant formatter, linter, type checker, or compiler check.
2. Run focused tests related to the changed code.
3. Expand to broader tests only when justified.
4. Report validation failures clearly and distinguish pre-existing failures from failures caused by the change.

Do not claim a change is validated unless the relevant command actually ran successfully.

## Git

- Do not commit, push, reset, rebase, or discard user changes unless explicitly requested.
- Treat staged changes as intentional and potentially important.
- Do not overwrite or unstage existing staged work.
- Inspect diffs narrowly and avoid touching unrelated files.
- Never assume an uncommitted working tree is disposable.

## Communication

When reporting code-search results:

- Show the file path and relevant line numbers.
- Prefer concise source excerpts over raw JSON.
- Explain the structural pattern used when `ast-grep` is involved.
- If a search may be incomplete because a parser or language is unsupported, say so.

When making changes:

- State what changed.
- State which files changed.
- State what validation was run and whether it passed.
- Mention material limitations or unresolved issues without padding the response.
