#!/usr/bin/env bash
# Creates the planned issue backlog for stellaryard-contracts in one run.
#
#   ./scripts/create-issues.sh [owner/repo]
#
# Titles are commit-style, bodies carry Summary / Acceptance Criteria /
# Tech Stack. Re-running skips issues whose title already exists.
set -euo pipefail

REPO="${1:-StellarYard/stellaryard-contracts}"

create_issue() {
  local title="$1" labels="$2" body="$3"

  if gh issue list --repo "$REPO" --state all --limit 200 \
      --json title --jq '.[].title' | grep -Fxq "$title"; then
    echo "skip (exists): $title"
    return 0
  fi

  gh issue create --repo "$REPO" --title "$title" --label "$labels" --body "$body" >/dev/null
  echo "created: $title"
}

# Labels this repo needs before the bodies resolve (matching stellaryard-core).
for l in "feature::a2eeef::New functionality" \
         "testing::0e8a16::Test coverage and integration tests" \
         "ready::0e8a16::Ready for contribution" \
         "critical-path::b60205::Blocks other issues or repos" \
         "good-first-issue::7057ff::Good for newcomers" \
         "documentation::0075ca::README, help text, spec updates"; do
  name="${l%%::*}"; color=$(echo "$l" | cut -d: -f3); desc="${l##*::}"
  gh label create "$name" --repo "$REPO" --color "$color" --description "$desc" --force >/dev/null 2>&1 || true
done

create_issue \
  "feat: add a second starter contract exercising token transfers" \
  "feature,critical-path" \
  "## Summary

\`counter\` proves deploy and invoke but never touches a token, which is what
almost every real Soroban contract does. A second starter contract that
moves a SAC token would give developers a working template for the pattern
they actually need: authorize, transfer, emit, settle.

## Why it matters

StellarYard's Contracts page lets a developer deploy a starter contract in
one click. Right now the only option is a counter, so the tool teaches
storage and auth but not value movement — the reason most people are
deploying contracts at all.

## Acceptance Criteria

- [ ] New contract under \`contracts/\` that transfers a token between two addresses
- [ ] Uses \`require_auth()\` on the sender; receiver cannot be drained
- [ ] Rejects transfers of zero and transfers exceeding balance with typed errors
- [ ] Unit tests cover success, insufficient balance, and missing auth
- [ ] \`cargo test\`, \`cargo fmt --check\`, and \`cargo clippy --all-targets -- -D warnings\` pass
- [ ] \`stellar contract build\` succeeds and the README artifact table is updated
- [ ] Workspace \`members\` updated

## Tech Stack

Rust, soroban-sdk 29, SAC token interface"

create_issue \
  "test: assert the emitted event payload decodes to the declared event type" \
  "testing,documentation" \
  "## Summary

Event tests currently assert that the right *number* of events fired and
which contract emitted them. They do not assert the payload, so a field
renamed or dropped from \`Incremented\` would pass CI while breaking any
off-chain indexer consuming it.

## Acceptance Criteria

- [ ] Each event test decodes the payload and asserts topic and data fields
- [ ] Covers \`Initialized\`, \`Incremented\`, \`CountSet\`, and \`Reset\`
- [ ] A deliberately wrong expected value fails the test (prove the assertion bites)
- [ ] \`cargo test\` passes

## Tech Stack

Rust, soroban-sdk 29 testutils, serde"

create_issue \
  "docs: document the deploy flow end to end from build to verified interaction" \
  "documentation,ready,good-first-issue" \
  "## Summary

The README lists build and deploy commands but a new contributor has no
single walkthrough that goes from a clean clone to a verified call against a
deployed contract.

## Acceptance Criteria

- [ ] Walkthrough: clone, \`cargo test\`, \`stellar contract build\`, generate an identity, fund it, deploy, invoke, verify on an explorer
- [ ] Every command was actually run and its real output is shown
- [ ] Testnet and local-network differences are called out explicitly
- [ ] The explorer link pattern for a deployed contract is included
- [ ] Broken or unimplemented steps are marked, not faked

## Tech Stack

MkDocs Material, Stellar CLI, Rust"

create_issue \
  "ci: build and test the workspace when a contract is added to members" \
  "testing,documentation" \
  "## Summary

CI runs \`cargo fmt\`, \`cargo clippy --all-targets\`, \`cargo test\`, and
\`stellar contract build\` at the workspace root, so a new contract is
covered automatically. This issue exists to make that guarantee explicit and
verified rather than assumed.

## Acceptance Criteria

- [ ] A test or CI step fails when a directory under \`contracts/\` is missing from workspace \`members\`
- [ ] The check runs in the existing \`Lint\` or \`Test\` job
- [ ] Documented in CONTRIBUTING.md alongside the add-a-contract steps

## Tech Stack

Rust, Cargo, GitHub Actions"

echo
echo "Done. Open issues in $REPO:"
gh issue list --repo "$REPO" --state open --limit 50 --json number,title \
  --jq '.[] | "  #\(.number) \(.title)"'
