<!-- ====== SEARCH RESULTS ====== -->
<!--
  This component is responsible for displaying the search results.
  It allows the user to select a file and preview its content.
  It also allows the user to scroll through the file content.
  It also allows the user to highlight text in the file content.
  It also allows the user to scroll through the file content.
-->

<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { highlightTextAction } from './highlightText';
  import type { SearchFile, SearchStats } from '../types/search';
  import { onMount } from 'svelte';

  export let files: SearchFile[] = [];
  export let basePath: string = '.';
  export let error: string | undefined = undefined;
  export let highlightColor: string = '#ffff00';
  export let searchQuery: string = '';
  export let useHorizontalScroll: boolean = false;
  export let stats: SearchStats | undefined = undefined;

  let selectedFile: string | null = null;
  let fileContent = '';
  let isPreviewLoading = false;
  let selectedLine: number | null = null;
  let lastScrolledLine: number | null = null;
  let lastScrolledFile: string | null = null;

  function getDisplayPath(fullPath: string, basePath: string): string {
    return fullPath.replace(basePath, '').replace(/^[\/\\]/, '');
  }

  async function loadFilePreview(filePath: string, lineNumber?: number) {
    try {
      // If it's the same file, just scroll to the line if provided
      if (selectedFile === filePath) {
        if (lineNumber) {
          scrollToLine(lineNumber);
        }
        return;
      }

      isPreviewLoading = true;
      selectedFile = filePath;
      selectedLine = lineNumber || null;

      const result = await invoke<string>('read_file', {
        path: filePath,
        lineNumber: lineNumber
      });

      if (result) {
        fileContent = result;
        
        // If a line number was specified, scroll to it after a short delay
        if (lineNumber) {
          setTimeout(() => {
            scrollToLine(lineNumber);
          }, 100);
        }
      }
    } catch (e) {
      console.error('Failed to load file preview:', e);
      error = e as string;
    } finally {
      isPreviewLoading = false;
    }
  }

  // Function to get file lines with correct line numbers
  function getFileLines(content: string, startLine: number = 1): { num: number; content: string }[] {
    return content.split('\n').map((line, index) => ({
      num: startLine + index,
      content: line
    }));
  }

  // Function to scroll to a specific line in the preview
  function scrollToLine(lineNum: number) {
    const previewContainer = document.querySelector('.file-preview');    
    if (!previewContainer) {
      console.log('Preview container not found in scrollToLine');
      return;
    }

    // Find the line element with the matching data-line attribute
    const lineElement = previewContainer.querySelector(`.line-${lineNum}`);
    
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

  // ============================== Devider of the left and right panes ==============================
  let leftPaneWidth = localStorage.getItem('leftPaneWidth') || '50%';
  onMount(() => {
    document.documentElement.style.setProperty('--left-width', leftPaneWidth);
    console.log('leftPaneWidth', leftPaneWidth);
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
            {#if stats}
              <div class="stat-item">
                <span class="stat-label">Search:</span>
                <span class="stat-value">{(stats.search_time_ms).toFixed(3)}ms</span>
              </div>
              <div class="stat-item">
                <span class="stat-label">Total:</span>
                <span class="stat-value">{(stats.total_time_ms).toFixed(3)}ms</span>
              </div>
            {/if}
          </div>
        </div>
        
        <div class="results">
          {#each files as file}
            <div class="file-section">
              <div 
                class="file-header" 
                on:click={() => loadFilePreview(file.name)}
                on:keydown={(e) => e.key === 'Enter' && loadFilePreview(file.name)}
                role="button"
                tabindex="0"
              >
                <span class="file-icon">📄</span>
                <span class="file-name" title={file.name}>{getDisplayPath(file.name, basePath)}</span>
                <span class="match-count">{file.lines.length} matches</span>
              </div>
              <div class="file-content">
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
                          use:highlightTextAction={{ 
                            text: line.content.replace(/&quot;/g, '"').replace(/&amp;/g, '&').replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&apos;/g, "'"), 
                            query: searchQuery, 
                            color: highlightColor 
                          }} />
                  </div>
                {/each}
              </div>
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
        {:else if fileContent}
          <div class="file-preview">
            {#each getFileLines(fileContent) as line}
              <div 
                class="preview-line line-{line.num}"
                data-line={line.num}
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

</style> 