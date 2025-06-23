###  Strengths (nothing to change here, just reinforcing)

1. **Clear problem definition** – ✔
2. **Terminology** – ✔
3. **Path fidelity** – ✔
4. **Conflict handling** – ✔
5. **Security** – ✔

These are core strengths and show we’re aligned on the core design.

---

### Concerns & Questions

1. **Complexity jump**

   Agreed. The new system adds deliberate friction and “power user” mechanics. One mitigation could be introducing onboarding shortcuts in the GUI (e.g. pre-filled forms, templates), while keeping the CLI fully deliberate.

   **No doc change needed unless we want to note this in Future Work.**

2. **Storage overhead**

   This is worth mentioning in the doc as a trade-off. We should add a sentence explaining that Forge duplicates files by design to maintain data safety and reproducibility, and may later support hard-linking or copy-on-write when safe.

   **☑ Suggest adding this note in the directory layout or edge-cases section.**

3. **Command surface area**

   The verbosity is intentional (as we explained with `start packing`). If users find it excessive, we can allow shell aliases but won’t promote them officially.

   **☑ Might be worth a line in “Rationale for Naming” to reinforce this design choice.**

4. **Git integration**

   This is a fair point. We should clarify how Git-related paths are handled. We already mark them in the manifest (`git_origin=true`) but should note that Forge does **not** manage Git operations — it expects the repo to be set up manually.

   **☑ Let’s add a short Git integration section to clarify this.**

---

### Suggestions

1. **`forge pack init` instead of `forge start packing`**

   We explicitly chose `start packing` for its deliberateness and awkwardness. Still, we could note in the appendix that `pack init` was considered but rejected for UX reasons.

   **☑ Worth adding to Appendix A.**

2. **Manifest metadata section**

   Great idea. Let’s add a `[metadata]` section to the manifest schema (e.g. `notes`, `tags`, `author`, `OS`). Even if we don’t use it yet, we can define the placeholder.

   **☑ Add to manifest section.**

3. **Dry-run mode**

   Strongly agree. This is already in our minds, but we should list it explicitly under stretch goals or in the edge-case table as a recommended safeguard.

   **☑ Add to Future Work or command notes.**