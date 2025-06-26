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

I want to add to add 
1) progress bar for the search process - replaced with spinning icon! ***DONE***
2) message when search line is empty or no path ***DONE***
3) reliable time stamp all the important parts in the process and print it to log ***DONE***
4) see why loading is taking too long - read word for possible cause
5) fix all problems in IDE
6) understand RG stats
7) in open screen - "No search result" should be replace with "enter search parameters and Search in fun"
    aLso the message "Select a file from the search results to preview" in open screen when no search was done - should be replaces
8) Add + for expand/collapse for the file neam and results. ***DONE***
    Add also expabd/collapse all ***DONE***
9) search in 1000 files every iteration ***DONE***
10) fix the vertical scroll should in the right pane also to scroll the results
12) Maybe feature - do tabs for new search
13) Maybe feature - open file in Klogg or extenal viewer (...)
14) Create VirtualList
