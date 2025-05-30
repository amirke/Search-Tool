<!--
  File: +page.svelte
  Purpose: Main Svelte frontend component for the Tauri search tool.
  - Handles user input for search query and folder selection.
  - Invokes Rust backend via Tauri to perform ripgrep search.
  - Displays results and errors.
  - Uses SearchResults component to render output.
  Special: Uses Tauri's invoke API to call backend command 'search_text'.
-->

<script lang="ts">
  // onMount: When the component mounts, automatically run a search with the default query and path.
  // Used by: Svelte lifecycle, triggers search on load.
  // Special: Results are set from backend, errors are caught and displayed.
  import { onMount } from 'svelte';
  import SearchResults from '../components/SearchResults.svelte'; // Renders the search results, expects 'results' and 'searchPath' props
  import { invoke } from '@tauri-apps/api/tauri'; // Used to call Rust backend commands

  let results: string = '';
  let searchPath = 'C:/Projects/Vin/TOP/Arinc818_Out/ARINC_818/ARINC818_design/ARINC818/ARINC818_Controller/Receive_A818'; // Default path for demo
  let query = 'when'; // Default query for demo
  let error = '';

  // AUTOMATE: Run search on mount
  // This will call the backend 'search_text' command with the default query and path.
  // Sets 'results' or 'error' accordingly.
  onMount(async () => {
    try {
      results = await invoke('search_text', { query, path: searchPath });
    } catch (e) {
      error = e + '';
    }
  });
</script>

<main>
  <h1>🔍 Fast Text Search (ripgrep)</h1>
  {#if error}
    <div class="error">{error}</div>
  {/if}
  <SearchResults {results} {searchPath} /> <!-- Passes results and path to SearchResults component -->
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
</style>
