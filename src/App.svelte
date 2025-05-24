<script>
  import { invoke } from '@tauri-apps/api/tauri';
  let query = '';
  let path = '.';
  let results = '';
  let error = '';

  async function search() {
    results = '';
    error = '';
    try {
      results = await invoke('search_text', { query, path });
    } catch (e) {
      error = e;
    }
  }
</script>

<main>
  <h1>🔍 חיפוש טקסט מהיר (ripgrep)</h1>

  <input placeholder="מה לחפש?" bind:value={query} />
  <input placeholder="נתיב תיקייה (למשל . או C:\\Users\\...)" bind:value={path} />
  <button on:click={search}>חפש</button>

  {#if error}
    <pre style="color: red;">{error}</pre>
  {:else if results}
    <pre style="white-space: pre-wrap;">{results}</pre>
  {/if}
</main>

<style>
  main {
    max-width: 700px;
    margin: auto;
    padding: 2rem;
    font-family: sans-serif;
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
