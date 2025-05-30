# Next Issues & Future Work

## 1. Display Grep Stats in Frontend
- [ ] Pass search statistics (files scanned, matches, duration, etc.) from Rust backend to Svelte frontend.
- [ ] Parse and display these stats in the UI (see `src/utils/resultParser.ts`).

## 2. Enhanced Result Navigation
- [ ] Implement keyboard and/or button navigation to move between search results.
- [ ] When navigating, keep the selected result centered in the view and visually emphasize the selected row.

## 3. Add Logging to Executable
- [ ] Ensure the Rust backend writes detailed logs to a file (e.g., `search_tool_debug.log`).
- [ ] Optionally, provide a way for users to view or export logs from the UI.

## 4. Code Documentation & Comments
- [ ] Add file-level doc comments to all `.rs`, `.ts`, and `.svelte` files, describing their purpose and usage.
- [ ] For each function, add comments explaining its role, parameters, and cross-file usage.
- [ ] Highlight any special parameters or cross-file dependencies.

## 5. General Improvements
- [ ] Improve error handling and user feedback in the UI.
- [ ] Consider packaging as an MSI installer for easier distribution.
- [ ] Explore options for trusted code signing for public release. 