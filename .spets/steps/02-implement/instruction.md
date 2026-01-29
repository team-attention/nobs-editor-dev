# Implement Step

Execute the approved plan task by task. Follow the plan **exactly** unless blocked.

## Core Rules

1. **Follow the plan** — execute tasks in the specified order. Deviate only if blocked, and document why.
2. **One task at a time** — mark in_progress, implement, verify, then mark complete before moving on.
3. **Verify before claiming done** — run the verification command from the plan. No "should work" or "probably passes."
4. **Test-first for new functionality** — write a failing test, make it pass, then refactor. Skip TDD only for non-code tasks.
5. **Follow existing patterns** — match the style, structure, and conventions of the codebase. Reuse existing utilities.
6. **YAGNI** — only implement what's in the plan. No extra features, no unrelated refactoring, no speculative abstractions.

## Process

### For Each Task

1. **Prepare:** Read the task, check dependencies are complete, review the target file.
2. **Implement:** Make the change. For new functionality, write a failing test first.
3. **Verify:** Run the verification command from the plan. Check the output confirms success.
4. **Document:** Record what was done, files changed, and verification evidence.

### When Blocked

- Missing info or unclear instructions → **stop and ask**, don't guess.
- 3+ failed fix attempts → question the approach, not just the code.
- Scope growing beyond plan → stop, document, continue with planned tasks only.

### Final Verification

After all tasks:
- Run full test suite, linter, and build.
- Check every acceptance criterion from the plan.

## Output

Follow the template. Skip sections that don't apply.
