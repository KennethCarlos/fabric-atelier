# Windsurf Setup Guide for Fabric Atelier

This guide explains how to configure Windsurf for optimal development on this project.

---

## Quick Setup

### 1. Access Customizations

Click the **Customizations** icon in the top-right of Cascade panel, or use "Windsurf - Settings" in the bottom-right corner.

### 2. Add Rules

Navigate to **Rules** panel and add the rules from `WINDSURF_RULES.md`:

**Priority Order** (add in this sequence):
1. Architecture Standards (Always On)
2. Rust Best Practices (Glob: `src/**/*.rs`)
3. Arrow/Parquet Operations (Glob: `src/arrow/**/*.rs, src/vector/**/*.rs`)
4. MCP Protocol (Glob: `src/mcp/**/*.rs`)
5. Pattern Management (Glob: `src/fabric/**/*.rs`)
6. Build System (Glob: `build.rs, src/build/**/*.rs`)
7. Testing Standards (Glob: `tests/**/*.rs, src/**/tests.rs`)
8. Documentation (Glob: `docs/**/*.md`)
9. Git Commits (Manual)
10. Code Review (Manual)

### 3. Add Workflows

Navigate to **Workflows** panel and add workflows from `WINDSURF_WORKFLOWS.md`:

**Recommended Workflows**:
- New Module Creation
- Add New Error Type
- Implement New Pattern Feature
- Performance Optimization
- Release Preparation

---

## Rule Activation Modes Explained

### Always On
**When to use**: Core architectural principles that apply everywhere

**Examples**:
- File size limits (300 lines)
- Documentation requirements
- Error handling patterns
- Code organization standards

**Benefits**:
- Consistent enforcement
- No need to remember to activate
- Applies to all code changes

### Glob Pattern
**When to use**: Language, framework, or directory-specific rules

**Examples**:
- Rust-specific patterns for `src/**/*.rs`
- Arrow operations for `src/arrow/**/*.rs`
- Test conventions for `tests/**/*.rs`

**Benefits**:
- Automatic activation when working on matching files
- Context-aware assistance
- Reduces cognitive load

**Glob Syntax**:
```
*.rs                    # All Rust files in current dir
**/*.rs                 # All Rust files recursively
src/mcp/**/*.rs         # All Rust files under src/mcp/
{src,tests}/**/*.rs     # Multiple directories
```

### Manual (@mention)
**When to use**: Workflow-specific or occasional tasks

**Examples**:
- Git commit message formatting
- Code review checklists
- Release preparation steps

**Usage**:
```
@git-commits Please help me write a commit message for these changes
@code-review Review this PR against our standards
```

**Benefits**:
- On-demand activation
- Doesn't clutter always-on context
- Explicit control

### Model Decision
**When to use**: Context-dependent rules that AI should decide

**Examples**:
- "Apply security best practices when handling user input"
- "Use SIMD optimizations for performance-critical vector operations"
- "Add extra logging when implementing error handling"

**Benefits**:
- Intelligent activation
- Adapts to context
- Reduces manual activation

---

## Memory Management

### Auto-Generated Memories

Cascade automatically creates memories for:
- Project-specific patterns it discovers
- Repeated code structures
- Common error resolutions
- Frequently used commands

**Best Practices**:
- Review auto-generated memories periodically
- Edit or delete irrelevant ones
- Memories are workspace-specific

### Creating Memories Manually

Ask Cascade to create a memory:
```
"Create a memory that we use simsimd crate for SIMD operations"
"Remember that pattern loading should fallback to ~/.config/fabric/patterns"
"Create a memory of our error handling approach using thiserror and anyhow"
```

**When to create memories**:
- Project-specific conventions
- Important architectural decisions
- Frequently referenced information
- Team agreements

### Memory vs Rules

| Aspect | Memories | Rules |
|--------|----------|-------|
| **Creation** | Auto or manual prompt | Manual file creation |
| **Scope** | Workspace-specific | Global or workspace |
| **Activation** | AI decides relevance | Always/Glob/Manual/Model |
| **Format** | Natural language | Structured markdown |
| **Use Case** | Context & decisions | Standards & guidelines |

**Rule of Thumb**:
- **Memories**: "What we decided" (context, history)
- **Rules**: "How we do things" (standards, process)

---

## Recommended Configuration

### Essential Rules (Always On)

```markdown
<architecture>
- No file exceeds 300 lines
- Every public API documented
- Use thiserror for errors, anyhow for context
- No unwrap() in production code
</architecture>

<performance>
- Use #[inline] for hot path functions < 10 lines
- Prefer zero-copy operations
- Use rayon for data parallelism
</performance>

<documentation>
- Rustdoc for all public items
- Include examples for non-trivial APIs
- Explain WHY in comments, not WHAT
</documentation>
```

### File-Specific Rules (Glob)

**For `src/**/*.rs`**:
```markdown
<rust_standards>
- Types: PascalCase
- Functions: snake_case
- Constants: SCREAMING_SNAKE_CASE
- Use tokio for async
- Prefer async fn over impl Future
</rust_standards>
```

**For `src/arrow/**/*.rs`**:
```markdown
<arrow_operations>
- Use zero-copy operations
- Prefer columnar over row-wise
- Use SIMD compute kernels
- Memory-map large Parquet files
</arrow_operations>
```

### Workflow Rules (Manual)

**@git-commits**:
```markdown
Format: <type>: <subject>

Types: feat, fix, docs, style, refactor, perf, test, chore
Subject: imperative, no period, max 50 chars
Body: explain WHAT and WHY
```

---

## Advanced Configuration

### XML Tag Organization

Group related rules for better organization:

```markdown
<coding_standards>
1. Use early returns when possible
2. Prefer composition over inheritance
3. Keep functions focused and small
</coding_standards>

<testing_standards>
1. Test both success and error cases
2. Use descriptive test names
3. Mock external dependencies
</testing_standards>

<security>
1. Validate all user input
2. Use constant-time comparisons for secrets
3. Never log sensitive data
</security>
```

### Character Limits

- Each rule file: **12,000 characters max**
- If exceeding, split into multiple rules
- Use focused, specific rules over generic ones

### Rule Priority

When multiple rules apply:
1. Always On rules (highest priority)
2. Glob-matched rules
3. Model Decision rules
4. Manual rules (when @mentioned)

---

## Testing Your Configuration

### Verify Rules Are Active

1. Open a Rust file in `src/`
2. Check Cascade panel for active rules indicator
3. Ask: "What rules are currently active?"

### Test Glob Patterns

1. Open `src/mcp/server.rs`
2. Verify MCP Protocol rules activate
3. Open `src/arrow/schema.rs`
4. Verify Arrow rules activate

### Test Manual Rules

1. Type `@git-commits` in Cascade
2. Verify commit rules appear
3. Ask for commit message help

### Test Memory Creation

1. Ask: "Create a memory that we use notify crate for file watching"
2. Verify memory appears in Memories panel
3. Test recall: "What do we use for file watching?"

---

## Maintenance

### Weekly Review

- Review auto-generated memories
- Remove outdated or incorrect memories
- Update rules based on new patterns
- Add new workflows for repeated tasks

### Monthly Audit

- Check rule effectiveness
- Consolidate similar rules
- Update glob patterns if structure changes
- Archive unused workflows

### When Adding New Features

1. Update relevant rules
2. Add new glob patterns if needed
3. Create workflow for common tasks
4. Document in memories if needed

---

## Troubleshooting

### Rules Not Activating

**Check**:
- Rule activation mode is correct
- Glob pattern matches file path
- Rule is enabled (not disabled)
- Character limit not exceeded

**Fix**:
- Verify glob syntax
- Test with simpler pattern
- Check Windsurf logs

### Too Many Rules Active

**Symptoms**:
- Slow Cascade responses
- Conflicting suggestions
- Context overflow

**Solutions**:
- Use more specific glob patterns
- Move generic rules to Manual mode
- Split large rules into focused ones
- Disable rarely-used rules

### Memories Not Recalled

**Check**:
- Memory is in correct workspace
- Memory content is clear and specific
- Not too many similar memories

**Fix**:
- Rephrase memory more clearly
- Add keywords for better matching
- Consolidate similar memories

---

## Best Practices Summary

### Do's ✅

- Keep rules concise and specific
- Use glob patterns for context-specific rules
- Create memories for project decisions
- Review and update regularly
- Test rules after adding them
- Use XML tags for organization
- Document rule purpose

### Don'ts ❌

- Don't add generic rules (already in training)
- Don't exceed character limits
- Don't create overlapping rules
- Don't use vague language
- Don't forget to test glob patterns
- Don't accumulate unused memories
- Don't mix standards and context

---

## Quick Reference

### Add Rule
1. Customizations → Rules → + Workspace/Global
2. Paste rule content
3. Set activation mode
4. Save

### Add Workflow
1. Customizations → Workflows → + New
2. Paste workflow steps
3. Set trigger
4. Test

### Create Memory
```
"Create a memory that [important context]"
```

### Use Manual Rule
```
@rule-name [your request]
```

### Check Active Rules
```
"What rules are currently active?"
```

---

## Next Steps

1. **Add core rules** from `WINDSURF_RULES.md`
2. **Test glob patterns** by opening different files
3. **Create initial memories** for key decisions
4. **Add 2-3 workflows** you'll use most
5. **Start coding** and refine as you go

The configuration will evolve with the project. Start with essentials and add more as patterns emerge.
