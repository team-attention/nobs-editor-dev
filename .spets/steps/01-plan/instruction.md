# Plan Step

Create an actionable implementation plan. This is a **read-only exploration phase** — do not modify any files except the plan document.

## Core Rules

1. **Explore first** — find similar patterns in the codebase before designing anything. Cite `file:line` references.
2. **Small tasks** — each task targets one file, has a concrete verification command, and takes 2-5 minutes.
3. **Evidence over assumptions** — never guess file paths or patterns. Use Glob/Grep/Read to verify.
4. **YAGNI** — only plan what's explicitly needed. No speculative features or abstractions.

## Process

### 1. Understand the Request
- Identify core requirements, scope, and success criteria.
- If requirements are unclear, list specific questions in YAML frontmatter `open_questions`.

### 2. Explore the Codebase
**This is the most important phase — spend the majority of time here.**
- Find all files that need modification (exact paths).
- Identify similar existing features with `file:line` references.
- Review testing patterns, type definitions, error handling, and utilities to reuse.
- Document findings: what pattern to extract and why it matters.

### 3. Design the Approach
- Present the chosen approach with justification from codebase evidence.
- Note alternatives considered (briefly) and why they were rejected.
- Decide test strategy: TDD / tests-after / manual QA only.

### 4. Break Down into Tasks
- Each task: `[Verb] [What] in [File]`
- Include: file path, specific changes, verification command + expected output, dependencies.
- Group into logical phases if needed.

## Output

Follow the template. Skip sections that don't apply to this task.
