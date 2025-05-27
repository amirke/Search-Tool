<script lang="ts">
  import type { SearchFile } from '../types/search';
  import { getDisplayPath } from '../utils/pathUtils';
  import { readFile } from '../services/searchService';
  import { highlightTextAction } from './highlightText';

  export let files: SearchFile[] = [];
  export let basePath = '.';
  export let error = '';
  export let highlightColor = '#ffff00';
  export let searchQuery = '';
  export let useHorizontalScroll = false;
  export let scannedFiles: number | undefined;
  export let filesWithMatches: number | undefined;
  export let totalMatches: number | undefined;
  export let durationMs: number | undefined;

  let selectedFile: string | null = null;
  let fileContent = '';
  let isPreviewLoading = false;
  let selectedLine: number | null = null;

  // Function to scroll to a specific line in the preview
  function scrollToLine(lineNum: number) {
    const lineElement = document.querySelector(`.line-${lineNum}`);
    if (lineElement) {
      lineElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
      lineElement.classList.add('highlighted-line');
      setTimeout(() => {
        lineElement.classList.remove('highlighted-line');
      }, 2000);
    }
  }

  async function loadFilePreview(filePath: string, lineNum?: number) {
    selectedFile = filePath;
    isPreviewLoading = true;
    selectedLine = lineNum || null;
    try {
      fileContent = await readFile(filePath);
      if (lineNum) {
        // Wait for the content to be rendered
        setTimeout(() => scrollToLine(lineNum), 100);
      }
    } catch (e) {
      error = `Failed to load file preview: ${e}`;
      fileContent = '';
    } finally {
      isPreviewLoading = false;
    }
  }

  // Function to split file content into lines with line numbers
  function getFileLines(content: string): { num: number; content: string }[] {
    return content.split('\n').map((line, index) => ({
      num: index + 1,
      content: line
    }));
  }
</script>

<div class="split-view">
  <!-- Search results panel -->
  <div class="search-panel">
    {#if error}
      <pre class="error">{error}</pre>
    {:else if files.length > 0}
      <!-- Statistics display -->
      {#if scannedFiles !== undefined}
        <div class="statistics">
          Scanned files: {scannedFiles}, Found {filesWithMatches} files with {totalMatches} matches. Done in {durationMs} ms.
        </div>
      {/if}
      
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
                  class="search-line" 
                  on:click={() => loadFilePreview(file.name, parseInt(line.num))}
                  on:keydown={(e) => e.key === 'Enter' && loadFilePreview(file.name, parseInt(line.num))}
                  role="button"
                  tabindex="0"
                  data-query={searchQuery}
                  data-color={highlightColor}
                >
                  <span class="line-num">{line.num}:</span>
                  <span class="line-content {useHorizontalScroll ? 'scrollable' : 'no-scroll'}" use:highlightTextAction={{ text: line.content, query: searchQuery, color: highlightColor }} />
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
              data-query={searchQuery}
              data-color={highlightColor}
            >
              <span class="line-num">{line.num}:</span>
              <span class="line-content {useHorizontalScroll ? 'scrollable' : 'no-scroll'}" use:highlightTextAction={{ text: line.content, query: searchQuery, color: highlightColor }} />
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

<style>
  .split-view {
    display: flex;
    flex: 1;
    gap: 1rem;
    min-height: 0; /* Important for nested flexbox scrolling */
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
  }

  .highlighted-line {
    animation: highlight 2s ease-out;
  }

  @keyframes highlight {
    0% { background-color: var(--highlight-color, #ffff00); }
    100% { background-color: transparent; }
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
</style> 