<!-- ====== SEARCH FORM ====== -->
<!--
  This component is responsible for the search form.
  It allows the user to enter a search query, select a folder, and specify a file filter.
  It also allows the user to select a highlight color and toggle horizontal scrolling.
  It also allows the user to browse for a folder.
  It also allows the user to select a history item.
  It also allows the user to submit the form.
  
  // מטרת הקובץ
  // זו הקומפוננטה שאחראית על קליטת הקלט מהמשתמש:
  // מה לחפש (query)
  // באיזה תיקיה (path)
  // באיזו צבע להדגיש תוצאות
  // האם לאפשר גלילה אופקית
  // אילו קבצים לכלול (fileFilter)
  // להפעיל את החיפוש
  // ולהציג היסטוריית חיפושים קודמים

-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  const dispatch = createEventDispatcher<{
    submit: { query: string; path: string; fileFilter: string };
  }>();

  export let useHorizontalScroll = false;
  export let highlightColor = '#ffff00';

  let searchQuery = '';
  let searchPath = '.';
  let isSearching = false;
  let fileFilter = '';

  async function handleSubmit() {
    if (!searchQuery.trim()) return;
    
    isSearching = true;
    try {
      dispatch('submit', { query: searchQuery, path: searchPath, fileFilter });
    } finally {
      isSearching = false;
    }
  }

  async function handleBrowse() {
    try {
      const result = await invoke<string>('open_folder_dialog');
      if (result) {
        searchPath = result;
      }
    } catch (e) {
      console.error('Failed to open folder dialog:', e);
    }
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="search-form">
  <div class="search-row">
    <div class="search-inputs">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Enter search query..."
        disabled={isSearching}
      />
      <input
        type="text"
        bind:value={fileFilter}
        placeholder="File type filter (e.g. *.vhd, *.txt)"
        disabled={isSearching}
      />
      <button type="button" on:click={handleBrowse} disabled={isSearching}>
        Browse
      </button>
      <button type="submit" disabled={isSearching}>
        {isSearching ? 'Searching...' : 'Search'}
      </button>
    </div>
    <div class="search-options">
      <label class="option">
        <input type="checkbox" bind:checked={useHorizontalScroll} />
        Enable horizontal scroll
      </label>
      <label class="option">
        Highlight color:
        <input type="color" bind:value={highlightColor} />
      </label>
    </div>
  </div>
  <div class="path-display">
    <span class="path-label">Search in:</span>
    <span class="path-value" title={searchPath}>{searchPath}</span>
  </div>
</form>

<style>
  .search-form {
    margin-bottom: 1rem;
  }

  .search-row {
    display: flex;
    gap: 1rem;
    align-items: center;
    margin-bottom: 0.5rem;
  }

  .search-inputs {
    display: flex;
    gap: 0.5rem;
    flex: 1;
  }

  .search-options {
    display: flex;
    gap: 1rem;
    align-items: center;
  }

  .option {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.9rem;
  }

  input[type="text"] {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  input[type="color"] {
    width: 30px;
    height: 20px;
    padding: 0;
    border: 1px solid #ccc;
    border-radius: 4px;
  }

  button {
    padding: 0.5rem 1rem;
    background: #4a90e2;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  button:hover {
    background: #357abd;
  }

  button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .path-display {
    font-size: 0.9rem;
    color: #666;
  }

  .path-label {
    font-weight: bold;
    margin-right: 0.5rem;
  }

  .path-value {
    word-break: break-all;
  }
</style> 