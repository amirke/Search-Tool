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
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { logWithTime } from '../utils/logger';

  import { isSearchingfromSearchservice, lastDuration, expandResults } from '../stores/searchState';
    $: $isSearchingfromSearchservice; // מאפשר תגובה אוטומטית
    $: $lastDuration; // מאפשר תגובה אוטומטית


    import { afterUpdate } from 'svelte';


  const dispatch = createEventDispatcher<{
    submit: { query: string; path: string; fileFilter: string; caseSensitive: boolean; wholePhrase: boolean; wholeWords: boolean };
  }>();

  export let useHorizontalScroll = false;
  export let highlightColor = '#ffff00';
  export let caseSensitive = true;
  export let wholePhrase = true;
  export let wholeWords = false;

  export let searchQuery = '';
  export let searchPath = '';
  export let fileFilter = '';



  export let error_message = '';



  
  afterUpdate(() => {
    logWithTime('✅ SearchForm HTML was updated');
  });



  async function handleSubmit() {

    logWithTime('✅ handleSubmit was called');

    if (!searchQuery.trim() && !searchPath) {
      error_message = "Missing search text or path";
      return;
    } else  if (!searchQuery.trim()) {
      error_message = "Missing search text  (path is " + searchPath + ")";
      return;
    } else if (!searchPath) {
      error_message = "Missing search path (search text is " + searchQuery + ")";
      return;
    }

    

    try {

      error_message = ''; 
      dispatch('submit', { query: searchQuery, path: searchPath, fileFilter, caseSensitive, wholePhrase, wholeWords });
    } finally {
 
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



  onMount(() => {
    logWithTime("SearchForm component mounted");
  });



</script>

<form on:submit|preventDefault={handleSubmit} class="search-form">
  <div class="search-row">
    <div class="search-inputs">
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Enter search query..."
        disabled={$isSearchingfromSearchservice}
      />
      <input
        type="text"
        bind:value={fileFilter}
        placeholder="File type filter (e.g. *.vhd, *.txt)"
        disabled={$isSearchingfromSearchservice}
      />
      <button type="button" on:click={handleBrowse} disabled={$isSearchingfromSearchservice}>
        Browse
      </button>
      <button type="submit" disabled={$isSearchingfromSearchservice}>
        {$isSearchingfromSearchservice ? 'Searching...' : 'Search'}
      </button>
    </div>
    <div class="search-options">
      <label class="option small-checkbox">
        <input type="checkbox" bind:checked={wholeWords} />
        <span>🔤 Only whole words</span>
      </label>
      <label class="option small-checkbox">
        <input type="checkbox" bind:checked={caseSensitive} />
        <span>🔠 Case Sensitive</span>
      </label>
      <label class="option small-checkbox">
        <input type="checkbox" bind:checked={wholePhrase} />
        <span>🧩 Literal substring match</span>
      </label>
      <label class="option">
        Highlight color:
        <input type="color" bind:value={highlightColor} />
      </label>
      <label class="option">
        <input type="checkbox" bind:checked={useHorizontalScroll} />
        Enable horizontal scroll
      </label>

      <button 
        type="button" 
        class="expand-toggle-btn" 
        on:click={() => $expandResults = !$expandResults}
      >
        {$expandResults ? 'Expanded' : 'Collapsed'}
      </button>
    </div>
  </div>
  <div class="path-display">
    <span class="path-label">Search in:</span>
    <span class="path-value" title={searchPath}>{searchPath}</span>
  </div>


  <div class="status-container">
    {#if error_message}
      <div class="status status-error">Status: {error_message}</div>
    {:else if $isSearchingfromSearchservice}
      <div class="status status-searching">
        <span class="search-icon">🔍</span>
        Status: Searching {searchQuery} in {searchPath}
      </div>
    {:else if $lastDuration > 0}
      <div class="status status-complete">
        Search “{searchQuery}” in “{searchPath}” completed.<br />
        Took {$lastDuration.toFixed(3)} ms.
      </div>
  
    {:else}
      <div class="status status-ready">Status: Best Search Utility is ready</div>
    {/if}
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

  .small-checkbox input[type="checkbox"] {
    width: 14px;
    height: 14px;
    margin-right: 4px;
  }
  .small-checkbox span {
    font-size: 0.85rem;
  }

  .status-container {
    margin: 1rem 0;
    font-family: sans-serif;
  }

  .status {
    padding: 0.5rem;
    border-radius: 4px;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .status-error {
    color: #b00;
  }

  .status-searching {
    color: #007bff;
  }

  .status-ready {
    color: #080;
  }

  .search-icon {
    display: inline-block;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .expand-toggle-btn {
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
    background: #28a745;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    min-width: 80px;
    transition: background-color 0.2s;
  }

  .expand-toggle-btn:hover {
    background: #218838;
  }

  .expand-toggle-btn:active {
    background: #1e7e34;
  }

</style> 