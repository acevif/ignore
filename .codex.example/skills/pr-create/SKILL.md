---
name: pr-create
description: Create pull requests in this repo using gh, including preparing CHANGELOG entries, related issue references, and PR descriptions; use when asked to open, draft, or submit a PR or to run gh pr create.
---

# PR Create

## Overview

Create a PR with consistent changelog hygiene and a clean PR description. Use this workflow when preparing or running `gh pr create` in this environment.

## Workflow

1. Confirm readiness. Ensure `CHANGELOG.md` already has an entry with a placeholder PR number because the real PR number is often unknown. Check for related issues and include any relevant ones in the changelog entry even if they are not fully resolved or closed.
2. Set the PR title using the Conventional Commits first-line format.
3. Prepare the PR description carefully. Inline `\n` escapes often end up as literal text (backslash + n), so verify real newlines; write the body to a file and use `gh pr create --body-file /tmp/pr-body.md` or a heredoc that writes the file.
4. Create the PR. Run `gh pr create` with the prepared title and body (it will push as needed).
5. Replace the changelog placeholder with the actual PR number after creation.
