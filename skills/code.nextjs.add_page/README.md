# code.nextjs.add_page

Adds a route to a Next.js App Router project while reusing local layout and component conventions.

## Example AgentSpec

```yaml
skills: [code.nextjs.add_page]
workspace:
  type: code.git
scope:
  allow: ["src/app/courses/**", "src/components/**"]
verify:
  profile: web_runtime_smoke
  routes:
    - { path: "/courses", expect: 200 }
```

Success test: route file builds and runtime smoke passes. Failure test: route is placed under the wrong App Router path.
