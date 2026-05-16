# verifier.web_runtime_smoke

Runs build and runtime HTTP route checks so a changed web route is verified after the app starts.

## Example AgentSpec

```yaml
skills: [verifier.web_runtime_smoke]
verify:
  profile: web_runtime_smoke
  runtime:
    start: "npm run start"
    routes:
      - { path: "/courses", expect: 200 }
```

Success test: changed route returns the expected status. Failure test: build passes but runtime route check fails.
