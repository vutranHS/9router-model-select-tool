---
name: git-guardian-pro
description: Protect source repositories before substantial AI-generated edits by checking Git availability, repository state, project boundaries, sensitive files, and recovery options. Use before multi-file edits, refactors, dependency upgrades, migrations, generated-code changes, repository initialization, or destructive Git recovery.
---

# Git Guardian Pro

Preserve the user's source code and Git history. Prefer read-only inspection and a clear recovery path.

## 1. Check whether Git is available

Run:

```bash
git --version
```

If the executable is unavailable:

- Do not run any other Git command.
- Do not attempt to install Git or initialize a repository without an explicit user request.
- Explain that repository checkpoints are unavailable until Git is installed.
- Recommend making a normal filesystem backup before substantial edits.
- Provide platform-appropriate installation guidance only when useful:
  - macOS: Apple Command Line Tools or the user's preferred package manager.
  - Windows: Git for Windows, `winget`, or the user's managed software catalog.

Installing this skill does not require Git. Its Git-backed protection activates after Git becomes available.

## 2. Detect an existing repository

Run:

```bash
git rev-parse --show-toplevel
git status --short
```

If the first command succeeds:

- Treat the returned path as the repository boundary.
- Never run `git init`.
- Inspect `git status` and `git diff` before substantial edits.
- Preserve unrelated user changes.
- Warn when the worktree is already dirty, but continue with safe read-only checks.

## 3. Decide whether repository initialization is safe

Only consider initialization when no repository exists.

Require all of the following:

- The current directory is an isolated project directory.
- At least one strong project signal exists, such as a package/build manifest, source directory, test directory, or VCS metadata file.
- The directory is not the filesystem root, user home directory, or the storage folder itself, such as `Desktop`, `Documents`, `Downloads`, an iCloud/Dropbox/OneDrive/Google Drive root, a mounted drive root, or a network-share root.
- A project nested inside one of those folders may still be valid; do not reject it solely because an ancestor is named `Documents` or `Desktop`.
- The project size is reasonable after excluding dependency, cache, and build-output directories.
- `.gitignore` exists or the intended initial snapshot has been reviewed for generated files and secrets.
- The user explicitly approves initialization and the initial snapshot.

Common project signals include:

```text
package.json, pyproject.toml, requirements.txt, go.mod, Cargo.toml,
pom.xml, build.gradle, *.sln, *.csproj, composer.json, Gemfile,
mix.exs, pubspec.yaml, Package.swift, *.xcodeproj, project.godot,
*.uproject, Dockerfile, docker-compose.yml, Makefile, CMakeLists.txt,
src/, app/, lib/, tests/, backend/, frontend/
```

Before staging an initial snapshot, look for likely secrets and generated content, including:

```text
.env*, *.pem, *.key, *.p12, credentials*, secrets*,
node_modules/, target/, dist/, build/, .venv/, vendor/
```

If confidence is low, skip initialization and explain why.

## 4. Create checkpoints only with authorization

Before a large edit, recommend a checkpoint when the repository is clean or when the user has reviewed the current changes.

Do not automatically stage or commit. If the user explicitly asks for a checkpoint:

1. Show `git status --short`.
2. Review the relevant diff and untracked files.
3. Stage only the intended paths.
4. Commit with a clear message.

Never use `git add .` as a blind default.

## 5. Classify Git operations

Read-only inspection is normally safe:

```text
git status, git diff, git log, git show, git rev-parse, git branch --show-current
```

Local or remote state-changing operations require a user request or clear authorization:

```text
git init, git add, git commit, git tag, git switch, git checkout,
git merge, git rebase, git pull, git fetch, git push
```

Potentially destructive operations require explicit confirmation immediately before execution:

```text
git reset --hard, git clean, git checkout -- <path>,
git restore <path>, git branch -D, git tag -d,
git filter-branch, git filter-repo, force push
```

Never use a destructive command merely to simplify the working tree.

## 6. Recovery

When changes fail:

1. Inspect `git status` and `git diff`.
2. Identify exactly which paths belong to the failed change.
3. Prefer a targeted repair or revert.
4. Explain the effect of any restore, reset, or clean command before asking for confirmation.
5. Preserve unrelated modified and untracked files.

When Git is unavailable or no repository exists, use the application's configuration backup/restore feature where applicable and recommend a normal project-directory backup before continuing.
