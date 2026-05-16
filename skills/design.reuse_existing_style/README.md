# design.reuse_existing_style

Reuses existing components, spacing, tokens, and layout patterns before adding new UI primitives.

## Example AgentSpec

```yaml
skills: [design.reuse_existing_style]
scope:
  allow: ["src/app/**", "src/components/**"]
verify:
  commands: ["npm run build"]
```

Success test: new UI follows existing component conventions. Failure test: a duplicate one-off component pattern is introduced.
