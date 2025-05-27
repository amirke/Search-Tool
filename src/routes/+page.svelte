<!--
  This is the main Svelte frontend component for the Tauri application.
  It provides a minimal UI for:
  - Entering a search query
  - Selecting a folder via a system dialog
  - Executing the `ripgrep` search via Rust backend (using Tauri `invoke`)
  - Displaying results or errors
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import SearchForm from '../components/SearchForm.svelte';
  import SearchResults from '../components/SearchResults.svelte';
  import { search, checkPortStatus } from '../services/searchService';
  import type { SearchResult } from '../types/search';

  let query = '';
  let path = '.';
  let highlightColor = '#ffff00';
  let useHorizontalScroll = false;
  let fileFilter = '';
  let isLoading = false;
  let error = '';
  let searchResult: SearchResult | null = null; // Store the whole result

  async function handleSearch(event: CustomEvent<{ 
    query: string; 
    path: string; 
    highlightColor: string;
    useHorizontalScroll: boolean;
    fileFilter: string;
  }>) {
    const { query, path, highlightColor: newColor, useHorizontalScroll: newScroll, fileFilter: newFilter } = event.detail;
    highlightColor = newColor;
    useHorizontalScroll = newScroll;
    fileFilter = newFilter;
    isLoading = true;
    error = '';
    searchResult = null; // Clear previous results
    try {
      const result = await search({ query, path });
      searchResult = result; // Store the full result
      if (result.error) {
        error = result.error;
      }
    } catch (e) {
      error = `Search failed: ${e}`;
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    checkPortStatus();
  });

  // Function to handle opening the About window (will implement later)
  function openAboutWindow() {
    console.log('Open About window'); // Placeholder
  }
</script>

<main>
  <h1>🔍 Fast Text Search (ripgrep)</h1>

  <SearchForm
    bind:query
    bind:path
    bind:highlightColor
    bind:useHorizontalScroll
    bind:fileFilter
    {isLoading}
    on:search={handleSearch}
  />

  {#if searchResult}
    <SearchResults
      files={searchResult.files}
      basePath={path}
      error={searchResult.error || ''}
      {highlightColor}
      searchQuery={query}
      {useHorizontalScroll}
      scannedFiles={searchResult.scannedFiles}
      filesWithMatches={searchResult.filesWithMatches}
      totalMatches={searchResult.totalMatches}
      durationMs={searchResult.durationMs}
    />
  {:else if error}
    <div class="error">{error}</div>
  {/if}

  <button class="about-button" on:click={openAboutWindow}>About</button>
</main>

<style>
  main {
    max-width: 100%;
    height: 100vh;
    margin: 0;
    padding: 1rem;
    font-family: system-ui, -apple-system, sans-serif;
    display: flex;
    flex-direction: column;
  }

  h1 {
    margin: 0 0 1rem 0;
    font-size: 1.5rem;
  }

  .error {
    color: #d32f2f;
    padding: 1rem;
    background: #ffebee;
    border-radius: 4px;
    margin: 1rem 0;
  }

  .about-button {
    margin-top: 1rem;
    padding: 0.5rem 1rem;
    font-size: 0.9rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    background: #f0f0f0;
    color: #333;
  }

  .about-button:hover {
    background: #e0e0e0;
  }
</style>
