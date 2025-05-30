<script lang="ts">
  import type { SearchFile } from '../types/search';
  import { getDisplayPath } from '../utils/pathUtils';
  import { readFile } from '../services/searchService';
  import { highlightTextAction } from './highlightText';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';
  import { readTextFile } from '@tauri-apps/api/fs';
  import { parseResults } from '../utils/resultParser';

  export let results: string = '';
  export let searchPath: string = '';

  let parsedResults: any[] = [];
  let selectedFile: string | null = null;
  let selectedLine: number | null = null;
  let fileContent: string = '';
  let lastPreviewedFile: string | null = null;
  let lastPreviewedLine: number | null = null;
  let highlightTimeout: number | null = null;
  let searchStats = {
    scanned: 0,
    files: 0,
    matches: 0,
    duration: 0
  };

  $: {
    console.log('Raw results received:', results);
    if (results) {
      try {
        const parsed = parseResults(results);
        console.log('Parsed results:', parsed);
        parsedResults = parsed.results;
        searchStats = parsed.stats;
        console.log('Updated searchStats:', searchStats);
      } catch (error) {
        console.error('Error parsing results:', error);
      }
    }
  }

  // Function to split file content into lines with line numbers
  function getFileLines(content: string, selectedLine: number | null): { num: number; content: string }[] {
    console.log('getFileLines called with selectedLine:', selectedLine);
    const lines = content.split('\n');
    console.log('Total lines from content:', lines.length);
    
    // If no selected line, just return all lines with sequential numbers
    if (selectedLine === null) {
        return lines.map((line, i) => ({ num: i + 1, content: line }));
    }
    
    // Calculate the start line number for the preview
    const startLineNum = Math.max(1, selectedLine - 10);
    console.log('Start line number:', startLineNum);
    
    // Return lines with their actual line numbers
    return lines.map((line, i) => {
        const lineNum = startLineNum + i;
        console.log(`Line ${i + 1}: actual number=${lineNum}, content length=${line.length}`);
        return { 
            num: lineNum, 
            content: line 
        };
    });
  }

  // Function to scroll to a specific line in the preview and highlight it
  function scrollToLine(lineNum: number) {
    console.log('scrollToLine called with lineNum:', lineNum);
    
    // Wait for DOM to update
    setTimeout(() => {
        const lineElement = document.querySelector(`[data-line="${lineNum}"]`);
        console.log('Found line element:', lineElement);
        
        if (lineElement) {
            // Remove any existing highlight
            document.querySelectorAll('.jumped-line').forEach(el => {
                el.classList.remove('jumped-line');
            });
            
            // Add highlight
            lineElement.classList.add('jumped-line');
            
            // Scroll to the line
            lineElement.scrollIntoView({ behavior: 'smooth', block: 'center' });
            console.log('Scrolled to line:', lineNum);
            
            // Remove highlight after 1 second
            setTimeout(() => {
                lineElement.classList.remove('jumped-line');
            }, 1000);
        } else {
            console.error('Could not find line element for line:', lineNum);
            // Debug: List all available line numbers
            const allLines = document.querySelectorAll('[data-line]');
            console.log('Available line numbers:', Array.from(allLines).map(el => el.getAttribute('data-line')));
        }
    }, 100);
  }

  async function loadFilePreview(filePath: string, lineNum: number) {
    console.log('loadFilePreview called with:', { filePath, lineNum });
    
    try {
        // If we're already showing this file, just scroll to the new line
        if (lastPreviewedFile === filePath) {
            console.log('Same file, just scrolling to line:', lineNum);
            selectedLine = lineNum;
            scrollToLine(lineNum);
            return;
        }
        
        console.log('Loading new file content');
        fileContent = await readFile(filePath, lineNum);
        console.log('File content loaded, length:', fileContent.length);
        
        lastPreviewedFile = filePath;
        selectedLine = lineNum;
        
        // Wait for DOM to update with new content
        setTimeout(() => {
            scrollToLine(lineNum);
        }, 100);
    } catch (error) {
        console.error('Error loading file preview:', error);
        fileContent = `Error loading file: ${error}`;
    }
  }
</script>

<div class="search-results">
  {#if parsedResults.length > 0}
    <div class="stats-bar">
      <span>Scanned {searchStats.scanned} files</span>
      <span>Found {searchStats.matches} matches in {searchStats.files} files</span>
      <span>Search took {searchStats.duration}ms</span>
    </div>
    <div class="results-container">
      <div class="results-list">
        {#each parsedResults as result}
          <div 
            class="result-item" 
            class:selected={selectedFile === result.file && selectedLine === result.line}
            on:click={() => loadFilePreview(result.file, result.line)}
          >
            <div class="file-path">{result.file}</div>
            <div class="line-number">Line {result.line}</div>
            <div class="line-content">{result.content}</div>
          </div>
        {/each}
      </div>
      {#if selectedFile}
        <div class="preview-panel">
          <div class="preview-header">
            <span class="file-name">{selectedFile}</span>
            <span class="line-number">Line {selectedLine}</span>
          </div>
          <div class="file-preview">
            {#each getFileLines(fileContent, selectedLine) as line}
              <div 
                class="preview-line" 
                class:jumped-line={line.num === selectedLine}
                data-line={line.num}
              >
                <span class="line-number">{line.num}</span>
                <span class="line-content">{line.content}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="no-results">No results found</div>
  {/if}
</div>

<style>
  .search-results {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .stats-bar {
    display: flex;
    gap: 1rem;
    padding: 0.5rem;
    background-color: var(--background-secondary);
    border-bottom: 1px solid var(--border-color);
    font-size: 0.9rem;
    color: var(--text-secondary);
  }

  .results-container {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .results-list {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
  }

  .result-item {
    padding: 0.5rem;
    border-bottom: 1px solid var(--border-color);
    cursor: pointer;
  }

  .result-item:hover {
    background-color: var(--background-hover);
  }

  .result-item.selected {
    background-color: var(--background-selected);
  }

  .file-path {
    font-weight: bold;
    margin-bottom: 0.25rem;
  }

  .line-number {
    color: var(--text-secondary);
    font-size: 0.9rem;
  }

  .line-content {
    font-family: monospace;
    white-space: pre;
    overflow-x: auto;
  }

  .preview-panel {
    flex: 1;
    display: flex;
    flex-direction: column;
    border-left: 1px solid var(--border-color);
  }

  .preview-header {
    padding: 0.5rem;
    background-color: var(--background-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .file-preview {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    font-family: monospace;
  }

  .preview-line {
    display: flex;
    gap: 1rem;
    padding: 0.25rem 0;
  }

  .preview-line .line-number {
    min-width: 3rem;
    text-align: right;
    user-select: none;
  }

  .preview-line .line-content {
    flex: 1;
    white-space: pre;
  }

  .jumped-line {
    background-color: var(--highlight-color);
    transition: background-color 0.3s ease;
  }

  .no-results {
    padding: 1rem;
    text-align: center;
    color: var(--text-secondary);
  }
</style> 