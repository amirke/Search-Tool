<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { browseFolder } from '../services/searchService';
  import { writable } from 'svelte/store';

  export let query = '';
  export let path = '.';
  export let isLoading = false;
  
  export let highlightColor = '#ffff00'; // Default yellow color
  export let useHorizontalScroll = false;
  export let fileFilter = '';

  // Create a store for search history
  const searchHistory = writable<string[]>([]);
  let showHistory = false;
  let historyItems: string[] = [];

  // Subscribe to search history changes
  searchHistory.subscribe(items => {
    historyItems = items;
  });

  const dispatch = createEventDispatcher<{
    search: { 
      query: string; 
      path: string; 
      highlightColor: string;
      useHorizontalScroll: boolean;
      fileFilter: string;
    };
  }>();

  async function handleBrowse() {
    const folder = await browseFolder();
    if (folder) {
      path = folder;
    }
  }

  function handleSubmit() {
    // Add to search history if not empty
    if (query.trim()) {
      searchHistory.update(history => {
        const newHistory = [query, ...history.filter(q => q !== query)].slice(0, 10);
        return newHistory;
      });
    }
    
    dispatch('search', { 
      query, 
      path, 
      highlightColor,
      useHorizontalScroll,
      fileFilter
    });
  }

  function handleHistorySelect(item: string) {
    query = item;
    showHistory = false;
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="search-form">
  <div class="input-group">
    <div class="search-input-container">
      <input 
        type="text" 
        placeholder="Enter search query..." 
        bind:value={query}
        on:focus={() => showHistory = true}
        on:blur={() => setTimeout(() => showHistory = false, 200)}
        class="search-input"
      />
      {#if showHistory && historyItems.length > 0}
        <div class="history-dropdown">
          {#each historyItems as item}
            <div 
              class="history-item"
              on:mousedown={() => handleHistorySelect(item)}
              role="button"
              tabindex="0"
            >
              {item}
            </div>
          {/each}
        </div>
      {/if}
    </div>
    <input
      type="color"
      bind:value={highlightColor}
      class="color-picker"
      title="Select highlight color"
    />
    <button type="submit" class="search-button" disabled={isLoading}>
      {isLoading ? 'Searching...' : 'Search'}
    </button>
  </div>

  <div class="input-group">
    <input 
      type="text" 
      placeholder="Folder path..." 
      bind:value={path} 
      class="path-input"
    />
    <button type="button" on:click={handleBrowse} class="browse-button">Browse</button>
  </div>

  <div class="input-group">
    <input 
      type="text" 
      placeholder="File filter (e.g., *.txt, *.js, src/**/*.ts)" 
      bind:value={fileFilter} 
      class="filter-input"
    />
    <label class="scroll-toggle">
      <input 
        type="checkbox" 
        bind:checked={useHorizontalScroll}
      />
      <span>Horizontal Scroll</span>
    </label>
  </div>
</form>

<style>
  .search-form {
    margin-bottom: 1rem;
  }

  .input-group {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .search-input-container {
    position: relative;
    flex: 1;
  }

  .search-input, .path-input, .filter-input {
    width: 100%;
    padding: 0.75rem;
    font-size: 1rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    background: #fff;
  }

  .color-picker {
    width: 40px;
    height: 40px;
    padding: 0;
    border: 1px solid #ddd;
    border-radius: 4px;
    cursor: pointer;
  }

  .color-picker::-webkit-color-swatch-wrapper {
    padding: 0;
  }

  .color-picker::-webkit-color-swatch {
    border: none;
    border-radius: 2px;
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

  .search-button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .browse-button {
    background: #f0f0f0;
    color: #333;
  }

  .search-button:hover:not(:disabled) {
    background: #0062a3;
  }

  .browse-button:hover {
    background: #e0e0e0;
  }

  .scroll-toggle {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.75rem;
    background: #f0f0f0;
    border-radius: 4px;
    cursor: pointer;
  }

  .scroll-toggle input[type="checkbox"] {
    width: 1rem;
    height: 1rem;
  }

  .history-dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: white;
    border: 1px solid #ddd;
    border-radius: 4px;
    margin-top: 0.25rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    z-index: 1000;
  }

  .history-item {
    padding: 0.5rem 0.75rem;
    cursor: pointer;
  }

  .history-item:hover {
    background: #f5f5f5;
  }
</style> 