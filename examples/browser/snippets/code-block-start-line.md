```rust
let source = r##"  if (!session) {
    redirect("/login")
  }

  const projects = await db.project.findMany({
    where: { ownerId: session.user.id },
    orderBy: { updatedAt: "desc" },
  })

  return <ProjectList projects={projects} />
}"##;

code_block(source).start_line(995).title("app/dashboard/page.tsx")
```
