<!--
  This is the main Svelte frontend component for the Tauri application.
  It provides a minimal UI for:
  - Entering a search query
  - Selecting a folder via a system dialog
  - Executing the `ripgrep` search via Rust backend (using Tauri `invoke`)
  - Displaying results or errors
-->

<script lang="ts">
  // Import lifecycle hook from Svelte
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  // Import child components
  import SearchForm from '../components/SearchForm.svelte';
  import SearchResults from '../components/SearchResults.svelte';
  import { lastDuration, lastTimestamp, expandResults } from '../stores/searchState';


  // Import the shape of the search result object
  import type { SearchFile, SearchStats } from '../types/search';


  import { isSearchingfromSearchservice } from '../stores/searchState';
  import { logWithTime } from '../utils/logger';

  import { afterUpdate } from 'svelte';

  // Define variables to store the search result and error message
  // ----- Reactive Variables -----
  let searchResult: { files: SearchFile[], stats?: SearchStats } = { files: [] };
  let error: string | undefined = undefined;
  let basePath = '.';
  let highlightColor = '#ffff00';
  let searchQuery = '';
  let useHorizontalScroll = false;
  let showAbout = false;
  let aboutText = '';
  let version = '';
  let fileFilter = '';
  let caseSensitive = true;
  let wholePhrase = true;
  let wholeWords = false;

  



  afterUpdate(() => {
    logWithTime('✅ +page.svelte HTML was updated');
  });


  /**
   * Handle the custom `search` event emitted from <SearchForm />
   * Extract all parameters and perform async call to Rust backend
   */

  async function handleSearch(query: string, path: string, fileFilter: string, caseSensitive: boolean, wholePhrase: boolean, wholeWords: boolean) {
    logWithTime("Search started (handleSearch)");
    const startTime = performance.now();

    try {
      isSearchingfromSearchservice.set(true);

      console.log('Query:', query);
      console.log('Path:', path);
      
      searchQuery = query; // Update the search query for highlighting
      basePath = path; // Update the base path for file display
      
      logWithTime("Calling Rust backend");
      const result = await invoke('search_text', { query, path, fileFilter, caseSensitive, wholePhrase, wholeWords });
      
      logWithTime("Processing search results");
      if (typeof result === 'string') {
        try {

/*          
          const duration = performance.now() - startTime;
          const now = new Date();
          const time = now.toISOString().split('T')[1].replace('Z', '');
  
          lastDuration.set(duration);
          lastTimestamp.set(time);
*/

          logWithTime("Search completed");
  //        console.log(`✅ Task completed in ${duration.toFixed(3)} ms`);

          isSearchingfromSearchservice.set(false);      
          const parsedResult = JSON.parse(result);
          
          // Parse the HTML content into files array
          const files: SearchFile[] = [];
          const htmlContent = parsedResult.html;
          
          if (htmlContent) {
            const lines = htmlContent.split('\n');
            
            for (const line of lines) {
              if (line.startsWith('<line')) {
                const fileMatch = line.match(/file="([^"]+)"/);
                const numMatch = line.match(/num="([^"]+)"/);

                const contentMatch = line.match(/>([^<]+)<\/line>/);
                
                 
                if (fileMatch && numMatch && contentMatch) {
                  const fileName = fileMatch[1];
                  const lineNum = numMatch[1];
                  const content = contentMatch[1];
                
                 
                  let file = files.find(f => f.name === fileName);
                  if (!file) {
                    file = { name: fileName, lines: [] };
                    files.push(file);

                  }

                  file.lines.push({ num: lineNum, content }); 

                }
              }
            }
          }
          
          searchResult = {
            files: files.map(f => ({ ...f, open: $expandResults })), // respect the checkbox state
            stats: parsedResult.stats
          };
          
          console.log('total matches:', searchResult.stats?.total_matches);
          console.log('matched lines:', searchResult.stats?.matched_lines);
          console.log('files searched:', searchResult.stats?.files_searched);
          console.log('search time:', searchResult.stats?.search_time_ms);
          console.log('total time:', searchResult.stats?.total_time_ms);
          console.log('error if any:', error);
          console.log('number of files:', searchResult.files.length);

          const duration = performance.now() - startTime;
          const now = new Date();
          const time = now.toISOString().split('T')[1].replace('Z', '');
  
          lastDuration.set(duration);
          lastTimestamp.set(time);
          console.log(`✅ Task completed in ${duration.toFixed(3)} ms`);





        } catch (e) {
          console.error('Failed to parse result:', e);
            //  error = e as string;
          error = String(e);

        }
      } else {
        console.log('=== UNEXPECTED RESULT TYPE ===');
        console.log('Result type:', typeof result);
        console.log('Result value:', result);
        console.log('=== END UNEXPECTED RESULT TYPE ===');
      }
    } catch (e) {
      logWithTime("Search failed");
      console.error('=== SEARCH ERROR ===');
      console.error(e);
      if (e instanceof Error) {
        console.error('Error details:', {
          message: e.message,
          stack: e.stack
        });
      }
      console.error('=== END SEARCH ERROR ===');
      //  error = e as string;
      error = String(e);

    }
  }

  async function openAboutWindow() {
    const [about, ver] = await invoke<[string, string]>('get_about_info');
    aboutText = about;
    version = ver;
    showAbout = true;
  }

  function closeAbout() {
    showAbout = false;
  }

  onMount(() => {
    logWithTime("App component mounted");
  });
</script>

<main>
  <div style="display: flex; justify-content: space-between; align-items: center;">
    <h1>🔍 KV Fast search (ripgrep)</h1>
    <button class="about-button" on:click={openAboutWindow}>About</button>
  </div>

  <!-- Search Form input fields -->
  <SearchForm 
    on:submit={({ detail }) => handleSearch(detail.query, detail.path, detail.fileFilter, detail.caseSensitive, detail.wholePhrase, detail.wholeWords)}
    bind:useHorizontalScroll
    bind:highlightColor
    bind:caseSensitive
    bind:wholePhrase
    bind:wholeWords
  />

  <!-- If result received, show it -->
  {#if searchResult}
    <SearchResults
      files={searchResult.files}
      basePath={basePath}
      error={error || ''}
      {highlightColor}
      searchQuery={searchQuery}
      {useHorizontalScroll}
      stats={searchResult.stats}
    />
  <!-- If only error occurred -->
  {:else if error}
    <pre class="error">{error}</pre>
  {/if}

  {#if showAbout}
    <div
      class="about-modal-backdrop"
      on:click={closeAbout}
      tabindex="0"
      role="dialog"
      aria-modal="true"
      on:keydown={(e) => { if (e.key === 'Escape') closeAbout(); }}
    >
      <div class="about-modal" on:click|stopPropagation>
        <h2>Search tool {version} - KV Labs</h2>
        <pre>{aboutText}</pre>
        <button on:click={closeAbout}>Close</button>
      </div>
    </div>
  {/if}
</main>

<!-- ====== STYLES ====== -->
<style>
  /* Main container */
  main {
    max-width: 100%;
    height: 100vh;
    margin: 0;
    padding: 1rem;
    font-family: sans-serif;
    display: flex;
    flex-direction: column;
  }

  /* Title */
  h1 {
    margin: 0 0 1rem 0;
    font-size: 1.5rem;
  }

  /* Error message */
  .error {
    color: red;
    margin: 1rem;
  }

  /* About button */
  .about-button {
    margin-left: 1rem;
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

  .about-modal-backdrop {
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: rgba(0,0,0,0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .about-modal {
    background: #fff;
    padding: 2rem;
    border-radius: 8px;
    min-width: 300px;
    max-width: 90vw;
    box-shadow: 0 2px 16px rgba(0,0,0,0.2);
    text-align: center;
  }

  .about-modal h2 {
    margin-top: 0;
  }

  .about-modal pre {
    text-align: left;
    background: #f4f4f4;
    padding: 1rem;
    border-radius: 4px;
    margin: 1rem 0;
    font-size: 1rem;
    max-height: 300px;
    overflow-y: auto;
  }
</style> 