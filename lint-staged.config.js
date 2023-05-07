export default {
  "**/*.rs": () => ["cargo fmt", "cargo clippy"],
  "**/*.md": (files) => `npx prettier --write ${files}`,
};
