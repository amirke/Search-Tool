<!-- ====== SEARCH RESULTS ====== -->
<!--
  This component is responsible for displaying the search results.
  It allows the user to select a file and preview its content.
  It also allows the user to scroll through the file content.
  It also allows the user to highlight text in the file content.
  It also allows the user to scroll through the file content.
-->

<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { highlightTextAction } from './highlightText';
  import type { SearchFile, SearchStats } from '../types/search';
  import { onMount } from 'svelte';
  import { tick } from 'svelte';
  import { logWithTime } from '../utils/logger';
  import { lastDuration, lastTimestamp, expandResults } from '../stores/searchState';
  import { afterUpdate } from 'svelte';
  



  export let files: SearchFile[] = [];
  export let basePath: string = '.';
  export let error: string | undefined = undefined;
  export let highlightColor: string = '#ffff00';
  export let searchQuery: string = '';
  export let useHorizontalScroll: boolean = false;
  export let stats: SearchStats | undefined = undefined;


  let selectedFile: string | null = null;
  let isPreviewLoading = false;
  let selectedLine: number | null = null;
  let lastScrolledLine: number | null = null;
  let lastScrolledFile: string | null = null;
  let previousLineNumber = 0;

  // let visibleOffset = 0;
  // let visibleCount = 0;
  let visibleRange = { offset: 0, count: 0 };
  let scrollTimeout: ReturnType<typeof setTimeout>;
  let lineHeight = 20;

  let ignoreScroll = false;
  let lineNumberBase = 0;  // Updated when you slice/remove lines

  let fileLines: string[] = [];
  let chunkSize = 100;
  let isLoading = false;

  let previewContainer: HTMLDivElement | null = null;

  let StartFlag = false;

  let previousExpand = false;


  let loadedLinesCount: Record<string, number> = {};
  const CHUNK_SIZE = 100;


  // React to global expand/collapse changes
  $: if (files?.length && $expandResults !== previousExpand) {
      files = files.map((f) => {
      f.open = $expandResults;
      return f;
      });
    files = files;
    previousExpand = $expandResults;

  }




  afterUpdate(() => {
    logWithTime('✅ SearchResults HTML was updated');
  });

//================   Display Lines   =================
//  This function is used to display the lines in the preview.
// reactively update the displayLines when the fileLines change.
//  It is also used to scroll to a specific line.
//  It is also used to scroll to the top or bottom of the preview.
//  It is also used to scroll to the top or bottom of the preview.
$: displayLines = fileLines.map((line, i) => ({ //
    num: lineNumberBase + i + 1,
    content: line
  }));

//================   Get Display Path   =================
//  This function is used to get the display path of the file.
//  It is also used to scroll to a specific line.
//  It is also used to scroll to the top or bottom of the preview.
//  It is also used to scroll to the top or bottom of the preview.
  function getDisplayPath(fullPath: string, basePath: string): string {
    return fullPath.replace(basePath, '').replace(/^[\/\\]/, '');
  }


//================   Load File Preview   =================
  // loadFilePreview is the main function that loads the file preview.
  // It is called when the user clicks on a file in the search results.
  // It is also called when the user scrolls to the top or bottom of the preview.
  // It is also called when the user clicks on a line in the preview.
  // It is also called when the user hovers over a line in the preview.
  // It is also called when the user focuses on a line in the preview.
  // It is also called when the user scrolls to the top or bottom of the preview.
  async function loadFilePreview(filePath: string, lineNumber?: number) {
  try {
  //  ignoreScroll = true;
    StartFlag = true;
    console.log('========loadFilePreview=========='); 
    const linesPerScreen = 2000; // Amir: this parametr can be changed
    const buffer = 20; // lines before and after
    const totalLinesToLoad = linesPerScreen+ (buffer * 2);              
    const halfChunk = Math.floor(totalLinesToLoad / 2);  // Half before and after
    const centerLine = lineNumber ?? 0;
    const safeOffset = Math.max(1, centerLine - halfChunk);  // Make sure we don't go below 0
    const countToLoad = linesPerScreen + buffer * 2;

    // Determine if we should skip reloading
    const isSameFile = selectedFile === filePath;
    const isLineVisible = (
      lineNumber === undefined ||
      (lineNumber >= visibleRange.offset &&
       lineNumber < visibleRange.offset + visibleRange.count)
    );

    console.log('isSameFile', isSameFile);
    console.log('isLineVisible', isLineVisible);
    console.log('lineNumber', lineNumber);  
    console.log('filePath', filePath); 
    console.log('safeOffset', safeOffset);
    console.log('visibleRange.offset', visibleRange.offset);
    console.log('visibleRange.count', visibleRange.count);


    const direction: 'up' | 'down' =
      previousLineNumber !== null && lineNumber !== undefined && lineNumber < previousLineNumber ? 'up' : 'down';
    console.log('direction', direction);

    if (isSameFile && isLineVisible) {
      if (lineNumber !== undefined) {
        scrollToLine(lineNumber, 'same');
        previousLineNumber = lineNumber;
        StartFlag = false;
        console.log('scrollToLine 1 ended, StartFlag = ', StartFlag);
      }
      return;
    }

    //  if the file is changed, clear the preview state (clean memory + UI state)
    if (!isSameFile) {
      fileLines = [];
      lineNumberBase = 0;
      visibleRange = { offset: 0, count: 0 };
      previousLineNumber = 0;
      selectedLine = null;
      ignoreScroll = false;
    }

    isPreviewLoading = true;
    selectedFile = filePath;
    selectedLine = lineNumber || null;



  ignoreScroll = true;

  const resultLines = await fetchChunk(safeOffset, countToLoad, filePath);
 

  console.log('opened from line', safeOffset, 'to line', safeOffset + countToLoad);


  if (resultLines.length) {
    fileLines = resultLines;
    lineNumberBase = safeOffset;
    visibleRange = { offset: safeOffset, count: countToLoad };

    setTimeout(() => {
      if (lineNumber) {
        scrollToLine(lineNumber, direction);
        previousLineNumber = lineNumber;
      }
    StartFlag = false;
    console.log('scrollToLine 2 ended, StartFlag = ', StartFlag);

    }, 100);
  }


  } catch (e) {
    console.error('Failed to load file preview:', e);
  //  error = e as string;
    error = String(e);
  } finally {
    isPreviewLoading = false;
  }
}

//================   Scroll To Line   =================
//  This function is used to scroll to a specific line in the preview.
//  It is also used to scroll to the top or bottom of the preview.
//  It is also used to scroll to a specific line.
//  It is also used to scroll to the top or bottom of the preview.
function scrollToLine(lineNum: number, direction: 'up' | 'down' | 'same' = 'down') {
    const previewContainer = document.querySelector('.file-preview');  // 
    console.log('Starting scrollToLine', lineNum);
    if (!previewContainer) {
      console.log('Preview container not found in scrollToLine');
    return;
      }

    // Find the line element with the matching data-line attribute
    const lineElement = previewContainer.querySelector(`[data-line="${lineNum}"]`);
    const lastlineElement = previewContainer.querySelector(`[data-line="${lineNum+68}"]`);



    if (direction === 'up') {
      if (lastlineElement) {
       lastlineElement.scrollIntoView({ behavior: 'instant', block: 'end' });

      } 
    }



    
    if (lineElement) {
      // Use instant scroll for initial load, smooth for subsequent
      lineElement.scrollIntoView({ 
        behavior: 'smooth', 
        block: 'center',
        inline: 'nearest'
      });
      
      // Add highlight
      lineElement.classList.add('highlighted-line');
      
      // Remove highlight after delay
      setTimeout(() => {
        lineElement.classList.remove('highlighted-line');
      }, 1500);
    } else {
      console.log('Line element not found for line:', lineNum);
    }
}




//================   Scroll Trigger   =================
//  This function is called when the user scrolls in the preview container.
//  It is used to load the previous or next chunk of lines.
//  It is also used to scroll to the top or bottom of the preview.
//  It is also used to scroll to a specific line.
//  It is also used to scroll to the top or bottom of the preview.
function onScroll(event: Event) {
  //if (1==1 ) return;
  const el = event.target as HTMLElement;

  const nearTop = el.scrollTop < 100;
  const nearBottom = el.scrollHeight - el.scrollTop - el.clientHeight < 100;
  const N_loadedFiles = fileLines.length < chunkSize;

  if (ignoreScroll) {
    if (nearTop || nearBottom || N_loadedFiles)  { 
      console.log('ignoreScroll: nearTop or nearBottom or N_loadedFiles', nearTop, nearBottom, N_loadedFiles);
      return;
    } else {
      ignoreScroll = false;
    }
  }

  // console.log('////////// on Scroll \\\\\\\\\\\\\\\\\\\\\\\\\\');


  if (nearTop) {  // 100 is the threshold for loading the previous chunk
    loadPreviousChunk();
  } else if (nearBottom) { // 100 is the threshold for loading the next chunk
    loadNextChunk();
  }
}

//================   Fetch Chunk   =================
//  This function is used to fetch a chunk of lines from the file.
//  It is also used to scroll to a specific line.
//  It is also used to scroll to the top or bottom of the preview.
//  It is also used to scroll to the top or bottom of the preview.
async function fetchChunk(offset: number, count: number, path?: string): Promise<string[]> {
  const filePath = path ?? selectedFile;
  if (!filePath) {
    console.error('No file path specified for fetchChunk');
    return [];
  }

  const result = await invoke<{ lines: string[] }>('read_file_mmap_chunk', {
    path: filePath,
    offset,
    count
  });

  console.log('opened chunk from line', offset, 'to line', offset + count);

  return result.lines;
}


//================   Load Next/Previous Chunk   =================
//  This function is used to load the next or previous chunk of lines from the file.
//  It is also used to scroll to a specific line.
//  It is also used to scroll to the top or bottom of the preview.
//  It is also used to scroll to the top or bottom of the preview.
async function loadNextChunk() {
  console.log('loadNextChunk');
  if (isLoading || !selectedFile) return; //this is the trigger for the scroll-load more
  isLoading = true;

  const start = lineNumberBase + fileLines.length;
  const result = await fetchChunk(start, chunkSize);

  fileLines = fileLines.concat(result); // result is string[]
  // Do not update lineNumberBase (we're appending to bottom)


  const maxLines = 300;
  
  if (fileLines.length > maxLines) {
    const over = fileLines.length - maxLines;
    console.log('over', over);
    console.log('lineNumberBase Before', lineNumberBase);
    console.log('fileLines.length Before', fileLines.length);  

    fileLines = fileLines.slice(over); // Remove from bottom
    lineNumberBase += over; 

    console.log('lineNumberBase After', lineNumberBase);
    console.log('fileLines.length After', fileLines.length);  
 
    // No need to change lineNumberBase
  }


  visibleRange = {
    offset: lineNumberBase,
    count: fileLines.length
  };

  isLoading = false;
  await tick();
}

async function loadPreviousChunk() {
  console.log('loadPreviousChunk');
  if (isLoading || !selectedFile || lineNumberBase === 0) return;
  isLoading = true;

  const newBase = Math.max(0, lineNumberBase - chunkSize);
  const result = await fetchChunk(newBase, chunkSize);

  const oldTopLineNum = lineNumberBase + 1;  // line currently at top before prepend

  fileLines = result.concat(fileLines); // prepend
  lineNumberBase = newBase;
 
  
  const maxLines = 300;
 
  if (fileLines.length > maxLines) {
    console.log('lineNumberBase Before', lineNumberBase);
    console.log('fileLines.length Before', fileLines.length);  
  
    fileLines = fileLines.slice(0, maxLines); // cut off from the end
    // ❗ lineNumberBase stays the same (you didn't remove from top)

    console.log('lineNumberBase After', lineNumberBase);
    console.log('fileLines.length After', fileLines.length);  
  }

 visibleRange = {
    offset: lineNumberBase,
    count: fileLines.length
  };

  isLoading = false;
  await tick();

  // scroll back to where user was
//  scrollToLine(lineNumberBase + chunkSize, 'same');
 // scrollToLine(oldTopLineNum, 'same');
}




// ============================== Devider of the left and right panes ==============================
//================   Resize Pane   =================
//  This function is used to resize the left pane.
//  It is also used to scroll to a specific line.
//  It is also used to scroll to the top or bottom of the preview.
//  It is also used to scroll to the top or bottom of the preview.
  let leftPaneWidth = localStorage.getItem('leftPaneWidth') || '50%';
  onMount(() => {
    document.documentElement.style.setProperty('--left-width', leftPaneWidth);
    console.log('leftPaneWidth', leftPaneWidth);
    logWithTime("SearchResults component mounted");

  });
  
  let isResizing = false;
  let startX: number;
  let startWidth: number;

  function startResize(e: MouseEvent) {
    isResizing = true;
    startX = e.clientX;
    const leftPane = document.querySelector('.left-pane') as HTMLElement;
    startWidth = leftPane.offsetWidth;
    
    // Prevent text selection during resize
    e.preventDefault();
    
    window.addEventListener('mousemove', resizePane);
    window.addEventListener('mouseup', stopResize);
  }

  function resizePane(e: MouseEvent) {
    if (!isResizing) return;
    
    const container = document.querySelector('.split-view') as HTMLElement;
    const containerWidth = container.offsetWidth;
    const deltaX = e.clientX - startX;
    const newWidth = Math.max(0, Math.min(containerWidth, startWidth + deltaX));
    const percent = (newWidth / containerWidth) * 100;
    
    // Ensure the width stays within bounds (0-100%)
    const boundedPercent = Math.max(0, Math.min(100, percent));
    leftPaneWidth = `${boundedPercent}%`;
    document.documentElement.style.setProperty('--left-width', leftPaneWidth);
    localStorage.setItem('leftPaneWidth', leftPaneWidth);
  }

  function stopResize() {
    isResizing = false;
    window.removeEventListener('mousemove', resizePane);
    window.removeEventListener('mouseup', stopResize);
  }

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// not in use from here...

  // let textAreaEl: HTMLTextAreaElement;

  // let onetimeflag = true;

  function handleLineHover(event: MouseEvent) {
  //  const line = (event.currentTarget as HTMLElement).dataset.line;
  //  console.log('Hovered line:', line);


  //  if (onetimeflag && line && parseInt(line) > 100) {
  //    onetimeflag = false;
  //    if (selectedFile) {
   //     loadMoreLines(selectedFile, parseInt(line), 200);
  //      loadMoreLines(selectedFile, 239, 50);
  //    }
  //  }
  }

function handleFocus(event: FocusEvent) {
//  const line = (event.currentTarget as HTMLElement).dataset.line;
//  console.log('Clicked line:', line);
  }


async function loadMoreLines(filePath: string, startOffset: number, count: number) {
//  const result = await invoke<{ lines: string[] }>('read_file_mmap_chunk', {
//    path: filePath,
//    offset: startOffset,
//    count
//  });

//  if (result && result.lines.length > 0) {
 //   lineNumberBase = -50;
 //   console.log('result', result.lines);
  //  fileContent = result.lines.join('\n');
 //   fileContent += '\n' + result.lines.join('\n'); //Append to the end
 //   fileContent = result.lines.join('\n') + '\n' + fileContent; //Prepend to the start


 //     console.log('fileContent', fileContent);
 //   const lineCount = fileContent.split('\n').length;
 //   console.log('lineCount', lineCount);

//   fileContent = fileContent.split('\n').slice(50).join('\n'); // remove the first 50 lines
/*    const lineCount2 = fileContent.split('\n').length;
    console.log('lineCount2', lineCount2);
    */
   // lineNumberBase = 150+13;

/*    fileLines = [...fileLines, ...result.lines]; // fileContent = fileLines.join('\n'); // re-assign for Svelte to re-render
    visibleRange.offset = Math.min(visibleRange.offset, startOffset);
    visibleRange.count = fileLines.length;*/
//  }
}


</script>

<div class="split-view">
  <div class="left-pane">
    <!-- Search results panel -->
    <div class="search-panel">
      {#if error}
        <pre class="error">{error}</pre>
      {:else if files.length > 0}
        <!-- Statistics display -->
        <div class="statistics">
          <div class="stats-grid">
            <div class="stat-item">
              <span class="stat-label">Files:</span>
              <span class="stat-value">{stats?.files_searched ?? files.length}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Matches:</span>
              <span class="stat-value">{stats?.total_matches ?? files.reduce((sum, file) => sum + (file.lines?.length || 0), 0)}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">Lines:</span>
              <span class="stat-value">{stats?.matched_lines ?? files.reduce((sum, file) => sum + (file.lines?.length || 0), 0)}</span>
            </div>
            <div class="stat-item">
              <span class="stat-label">⏱Durations:</span>
              <span class="stat-value">{$lastDuration.toFixed(3)} ms</span>
            </div>


           <!--  {#if stats}   -->
           {#if stats}   
              <div class="stat-item">
                <span class="stat-label">Search:</span>
                <span class="stat-value">{(stats?.search_time_ms ?? 0).toFixed(3)}ms</span>
              </div>
              <div class="stat-item">
                <span class="stat-label">Total:</span>
                <span class="stat-value">{(stats?.total_time_ms ?? 0).toFixed(3)}ms</span>
              </div>
            {/if}
          </div>
        </div>
        
        <div class="results">
          {#each files as file, i}
            <div class="file-section">
              <div 
                class="file-header" 
                role="button" tabindex="0"
                on:click={() => loadFilePreview(file.name)}
                on:keydown={(e) => e.key === 'Enter' && loadFilePreview(file.name)}
              >
                <!-- Expand arrow – CLICK HERE toggles collapse only -->
                <span
                  class="expand-icon"
                  role="button"
                  tabindex="0"
                  aria-expanded={file.open}
                  on:click={(e) => {
                    e.stopPropagation();
                    files[i] = { ...files[i], open: !files[i].open };
                    files = [...files]; // ← FORCE reactivity
                  }}
                  on:keydown={(e) => e.key === 'Enter' && (file.open = !file.open)}
                >
           <!--       {file.open ? '▼' : '▶️'} -->
                  {file.open ? '➖' : '➕'}
                </span>
                <span class="file-icon">📄</span>
                <span class="file-name" title={file.name}>{getDisplayPath(file.name, basePath)}</span>
                <span class="match-count">{file.lines.length} matches</span>
              </div>
       <!--      {#if file.open} -->

       
                <div class="file-content" hidden={!file.open}>
                    
                  {#each file.lines as line}

                    <div 
                      class="search-line line-{line.num}"
                      on:click={() => loadFilePreview(file.name, parseInt(line.num))}
                      on:keydown={(e) => e.key === 'Enter' && loadFilePreview(file.name, parseInt(line.num))}
                      role="button"
                      tabindex="0"
                      data-query={searchQuery}
                      data-color={highlightColor}
                    >
                      <span class="line-num">{line.num}:</span>
                      <span class="line-content {useHorizontalScroll ? 'scrollable' : 'no-scroll'}" 
                            use:highlightTextAction={{ text: line.content, query: searchQuery, color: highlightColor }} />
                    </div>
                  {/each}
                </div>
       <!--      {/if} -->
            </div>
          {/each}
        </div>

      {:else}
        <div class="no-results">No results found</div>
      {/if}
    </div>
  </div>

  <!-- Divider -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="resizer" on:mousedown={startResize}></div>

  <div class="right-pane">
    <!-- File preview panel -->
    <div class="preview-panel">
      {#if selectedFile}
        <div class="preview-header">
          <span class="preview-title">{getDisplayPath(selectedFile, basePath)}</span>
        </div>
        {#if isPreviewLoading}
          <div class="loading">Loading file preview...</div>
        {:else if displayLines.length > 0}
          <div class="file-preview" on:scroll={onScroll}>
            {#each displayLines as line (line.num)}
              <div 
                class="preview-line line-{line.num}"
                id={"line-" + line.num} 
                data-line={line.num}
                on:mouseover={handleLineHover}
                on:focus={handleFocus}
                role="option"  
                tabindex="0"   
                data-query={searchQuery}
                data-color={highlightColor}
              >
                <span class="line-num">{line.num}:</span>
                <span class="line-content {useHorizontalScroll ? 'scrollable' : 'no-scroll'}" 
                      use:highlightTextAction={{ text: line.content, query: searchQuery, color: highlightColor }} />
              </div>
            {/each}
          </div>
        {:else}
          <div class="no-preview">No preview available</div>
        {/if}
      {:else}
        <div class="no-file-selected">
          Select a file from the search results to preview
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  :root {
    --left-width: 50%;
  }

  .split-view {
    display: flex;
    height: 100vh;
    overflow: hidden;
    position: relative;
  }

  .search-panel, .preview-panel {
    flex: 1;
    overflow: auto;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: #fff;
  }

  .file-section {
    border-bottom: 1px solid #eee;
  }

  .file-header {
    padding: 0.5rem;
    background: #f5f5f5;
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .file-header:hover {
    background: #e9e9e9;
  }

  .file-icon {
    font-size: 1.2rem;
  }

  .file-name {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .match-count {
    color: #666;
    font-size: 0.9rem;
  }

  .file-content {
    padding: 0.5rem;
  }

  .search-line, .preview-line {
    display: flex;
    gap: 0.5rem;
    padding: 0.25rem 0;
    cursor: pointer;
  }

  .search-line:hover, .preview-line:hover {
    background: #f5f5f5;
  }

  .line-num {
    color: #666;
    min-width: 3rem;
    text-align: right;
    user-select: none;
  }

  .line-content {
    flex: 1;
    white-space: pre;
  }

  .line-content.scrollable {
    overflow-x: auto;
    text-overflow: clip;
  }

  .line-content.no-scroll {
    overflow-x: hidden;
    text-overflow: ellipsis;
  }

  .preview-header {
    padding: 0.5rem;
    background: #f5f5f5;
    border-bottom: 1px solid #ddd;
  }

  .preview-title {
    font-weight: bold;
  }

  .file-preview {
    margin: 0;
    padding: 0.5rem;
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 0.9rem;
    line-height: 1.4;
    height: calc(100vh - 100px); /* Adjust for header and padding */
    overflow-y: auto;
    scroll-behavior: smooth;
    position: relative;
    min-height: 200px;
    flex-grow: 1;
  }

  .preview-line {
    display: flex;
    gap: 0.5rem;
    padding: 0.25rem 0;
    cursor: pointer;
    transition: background-color 0.3s ease;
  }

  .preview-line:hover {
    background: #f5f5f5;
  }

  .highlighted-line {
    background-color: var(--highlight-color, #e0e0e0) !important;
    transition: background-color 0.3s ease;
  }

  .loading, .no-preview, .no-file-selected, .no-results {
    padding: 1rem;
    color: #666;
    text-align: center;
  }

  .error {
    color: #d32f2f;
    padding: 1rem;
    background: #ffebee;
    border-radius: 4px;
    margin: 1rem 0;
  }

  .left-pane {
    width: var(--left-width);
    overflow-y: auto;
    flex-shrink: 0;
  }

  .right-pane {
    flex: 1;
    overflow-y: auto;
    background-color: #f5f5f5;
    min-width: 0;
  }

  .resizer {
    width: 5px;
    background: #ccc;
    cursor: col-resize;
    flex-shrink: 0;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
  }

  .resizer:hover {
    background: #999;
  }

  /* Prevent text selection during resize */
  .split-view.resizing {
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
  }

  .statistics {
    padding: 0.5rem;
    background: #f5f5f5;
    border-bottom: 1px solid #ddd;
    margin-bottom: 0.5rem;
    font-size: 0.9rem;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
    gap: 0.5rem;
  }

  .stat-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.25rem;
  }

  .stat-label {
    color: #666;
    font-size: 0.85rem;
  }

  .stat-value {
    font-weight: 500;
    color: #333;
    font-size: 0.85rem;
  }

  .results {
    content-visibility: auto;
    contain-intrinsic-size: 0px 600px;
  }


</style> 