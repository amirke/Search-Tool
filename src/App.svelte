<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import SearchResults from './components/SearchResults.svelte';
  import type { SearchFile, SearchStats } from './types/search';

  let query = '';
  let path = '.';
  let results = '';
  let error = '';
  let files: SearchFile[] = [];
  let basePath = '.';
  let highlightColor = '#ffff00';
  let searchQuery = '';
  let useHorizontalScroll = false;
  let stats: SearchStats | undefined = undefined;

  async function search() {
    try {
      console.log('=== STARTING SEARCH ===');
      console.log('Query:', query);
      console.log('Path:', path);
      
      const result = await invoke('search_text', { query, path });
      console.log('=== RAW MESSAGE FROM RUST ===');
      console.log('Type:', typeof result);
      console.log('Full result:', JSON.stringify(result, null, 2));
      console.log('=== END RAW MESSAGE ===');
      
      if (typeof result === 'string') {
        try {
          const parsedResult = JSON.parse(result);
          console.log('=== PARSED RESULT ===');
          console.log(JSON.stringify(parsedResult, null, 2));
          console.log('=== END PARSED RESULT ===');
          
          if (parsedResult.files) {
            files = parsedResult.files;
            console.log('Files array length:', files.length);
            console.log('Files:', JSON.stringify(files, null, 2));
          }

          if (parsedResult.stats) {
            console.log('Stats:', JSON.stringify(parsedResult.stats, null, 2));
            stats = parsedResult.stats;
          }
          
          // Store the raw result for display
          results = JSON.stringify(parsedResult, null, 2);
        } catch (e) {
          console.error('Failed to parse result:', e);
          error = e as string;
        }
      } else {
        console.log('=== UNEXPECTED RESULT TYPE ===');
        console.log('Result type:', typeof result);
        console.log('Result value:', result);
        console.log('=== END UNEXPECTED RESULT TYPE ===');
      }
    } catch (e) {
      console.error('=== SEARCH ERROR ===');
      console.error(e);
      if (e instanceof Error) {
        console.error('Error details:', {
          message: e.message,
          stack: e.stack
        });
      }
      console.error('=== END SEARCH ERROR ===');
      error = e as string;
    }
  }
</script>

<main>
  <h1>🔍 חיפוש טקסט מהיר (ripgrep)</h1>

  <div class="container">
    <input type="text" bind:value={query} placeholder="Enter search query" />
    <input type="text" bind:value={path} placeholder="Enter path" />
    <button on:click={search}>Search</button>
  </div>

  {#if error}
    <pre class="error">{error}</pre>
  {/if}

  {#if results}
    <pre style="white-space: pre-wrap;">{results}</pre>
  {/if}

  <SearchResults 
    {files} 
    {basePath} 
    {error} 
    {highlightColor} 
    {searchQuery} 
    {useHorizontalScroll}
    {stats}
  />
</main>

<style>
  main {
    max-width: 700px;
    margin: auto;
    padding: 2rem;
    font-family: sans-serif;
  }

  .container {
    margin: 1rem;
    display: flex;
    gap: 1rem;
  }

  .error {
    color: red;
    margin: 1rem;
  }

  input, button {
    display: block;
    width: 100%;
    margin-top: 0.5rem;
    padding: 0.5rem;
    font-size: 1rem;
  }

  button {
    background: #007acc;
    color: white;
    border: none;
    cursor: pointer;
  }

  pre {
    background: #f4f4f4;
    padding: 1rem;
    margin-top: 1rem;
  }
</style>
