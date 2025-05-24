<!--
  This is the main Svelte frontend component for the Tauri application.
  It provides a minimal UI for:
  - Entering a search query
  - Selecting a folder via a system dialog
  - Executing the `ripgrep` search via Rust backend (using Tauri `invoke`)
  - Displaying results or errors
-->

<script lang="ts">
  // Importing Tauri's invoke function to call Rust commands
  import { invoke } from "@tauri-apps/api";

  // State variables
  let query = '';           // Search term entered by user
  let path = '.';           // Folder path (default: current folder)
  let results = '';         // Text result to be displayed
  let error = '';           // Any error message
  let statusMessage = '';

  // Function to open folder selection dialog via Rust backend
  async function browseFolder() {
    try {
      const folder = await invoke('open_folder_dialog') as string;
      if (folder) {
        path = folder;
      }
    } catch (err) {
      error = `Browse failed: ${err}`;
    }
  }

  // Function to send search request to backend (Rust) via Tauri
  async function search() {
    if (!query.trim()) {
      error = 'Please enter a search query';
      return;
    }
    if (!path.trim()) {
      error = 'Please select a folder to search in';
      return;
    }

    results = '';
    error = '';
    try {
      const searchParams = { query: query.trim(), path: path.trim() };
      const rawResults = await invoke('search_text', searchParams) as string;
      
      if (!rawResults || !rawResults.trim()) {
        error = 'No results found';
        return;
      }
      
      results = rawResults;
    } catch (e) {
      if (e instanceof Error) {
        error = e.message;
      } else if (typeof e === 'string') {
        error = e;
      } else {
        error = 'Unknown error occurred';
      }
    }
  }

  // Function to get relative path
  function getRelativePath(fullPath: string) {
    if (!path || path === '.') return fullPath;
    // Convert both paths to forward slashes for consistent comparison
    const normalizedPath = path.replace(/\\/g, '/');
    const normalizedFullPath = fullPath.replace(/\\/g, '/');
    
    // If the full path starts with the search path, remove it
    if (normalizedFullPath.startsWith(normalizedPath)) {
      return normalizedFullPath.slice(normalizedPath.length).replace(/^[\/\\]/, '');
    }
    
    // If we can't make it relative, return the last part of the path
    const parts = normalizedFullPath.split('/');
    return parts[parts.length - 1];
  }

  // Function to get display path
  function getDisplayPath(fullPath: string) {
    const relativePath = getRelativePath(fullPath);
    return relativePath.length > 50 ? '...' + relativePath.slice(-47) : relativePath;
  }

  // Function to parse the results into a structured format
  function parseResults(text: string) {
    try {
      const files: { name: string; lines: { num: string; content: string }[] }[] = [];
      const fileMap = new Map<string, { name: string; lines: { num: string; content: string }[] }>();
      
      const lines = text.split('\n');
      
      lines.forEach((line) => {
        if (line.startsWith('<line') && line.endsWith('</line>')) {
          try {
            const fileMatch = line.match(/file="([^"]+)"/);
            const numMatch = line.match(/num="(\d+)"/);
            const content = line.slice(line.indexOf('>') + 1, -7);
            
            if (fileMatch && numMatch) {
              const fileName = fileMatch[1];
              const lineNum = numMatch[1];
              
              if (!fileMap.has(fileName)) {
                fileMap.set(fileName, { name: fileName, lines: [] });
              }
              fileMap.get(fileName)?.lines.push({ num: lineNum, content });
            }
          } catch (e) {
            // Silently skip malformed lines
          }
        }
      });

      // Convert map to array and sort
      const sortedFiles = Array.from(fileMap.values());
      sortedFiles.sort((a, b) => a.name.localeCompare(b.name));
      return sortedFiles;
    } catch (e) {
      error = 'Failed to parse search results';
      return [];
    }
  }

  async function checkPortStatus() {
    try {
      const result = await invoke('ensure_port_1420_available') as string;
      console.log('[Frontend] ' + result);
    } catch (e) {
      const msg = 'Failed to check port: ' + (e instanceof Error ? e.message : e);
      console.log('[Frontend] ' + msg);
    }
  }

  import { onMount } from 'svelte';
  onMount(() => {
    checkPortStatus();
  });
</script>

<!-- User interface layout -->
<main>
  <h1>🔍 Fast Text Search (ripgrep)</h1>

  <!-- Search form -->
  <form on:submit|preventDefault={search} class="search-form">
    <div class="input-group">
      <input 
        type="text" 
        placeholder="Enter search query..." 
        bind:value={query} 
        class="search-input"
      />
      <button type="submit" class="search-button">Search</button>
    </div>

    <!-- Folder selection input and Browse button -->
    <div class="input-group">
      <input 
        type="text" 
        placeholder="Folder path..." 
        bind:value={path} 
        class="path-input"
      />
      <button type="button" on:click={browseFolder} class="browse-button">Browse</button>
    </div>
  </form>

  <!-- Conditional rendering for error or results -->
  {#if error}
    <pre class="error">{error}</pre>
  {:else if results}
    <div class="results">
      {#each parseResults(results) as file}
        <div class="file-section">
          <div class="file-header">
            <span class="file-icon">📄</span>
            <span class="file-name" title={file.name}>{getDisplayPath(file.name)}</span>
            <span class="match-count">{file.lines.length} matches</span>
          </div>
          <div class="file-content">
            {#each file.lines as line}
              <div class="search-line">
                <span class="line-num">{line.num}:</span>
                <span class="line-content">{line.content}</span>
              </div>
            {/each}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</main>

<!-- Styles -->
<style>
  main {
    max-width: 900px;
    margin: auto;
    padding: 2rem;
    font-family: system-ui, -apple-system, sans-serif;
  }

  .search-form {
    margin-bottom: 2rem;
  }

  .input-group {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 1rem;
  }

  .search-input, .path-input {
    flex: 1;
    padding: 0.75rem;
    font-size: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: #fff;
  }

  .search-button, .browse-button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .search-button {
    background: #007acc;
    color: white;
  }

  .browse-button {
    background: #f0f0f0;
    color: #333;
  }

  .search-button:hover {
    background: #0062a3;
  }

  .browse-button:hover {
    background: #e0e0e0;
  }

  .results {
    background: #fff;
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    overflow: hidden;
  }

  .file-section {
    border-bottom: 1px solid #e0e0e0;
  }

  .file-section:last-child {
    border-bottom: none;
  }

  .file-header {
    padding: 0.75rem 1rem;
    background: #f8f9fa;
    border-bottom: 1px solid #e0e0e0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .file-icon {
    font-size: 1.2em;
  }

  .file-name {
    font-weight: 500;
    color: #1a73e8;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    max-width: 70%;
  }

  .match-count {
    margin-left: auto;
    color: #666;
    font-size: 0.9em;
  }

  .file-content {
    padding: 0.5rem 0;
    overflow-x: auto;
  }

  .search-line {
    display: flex;
    padding: 0.25rem 1rem;
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 0.9em;
    line-height: 1.5;
    white-space: nowrap;
  }

  .line-num {
    color: #666;
    min-width: 3em;
    padding-right: 1rem;
    text-align: right;
    user-select: none;
  }

  .line-content {
    color: #333;
    flex: 1;
  }

  .error {
    color: #d32f2f;
    padding: 1rem;
    background: #ffebee;
    border-radius: 4px;
    white-space: pre-wrap;
  }
</style>
