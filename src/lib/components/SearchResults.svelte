<style>
  .results-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
  }

  .results-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background-color: var(--background-secondary);
    border-bottom: 1px solid var(--border-color);
  }

  .results-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .results-pane {
    flex: 1;
    overflow-y: auto;
    padding: 0.5rem;
    border-right: 1px solid var(--border-color);
  }

  .results-pane:last-child {
    border-right: none;
  }

  .result-item {
    margin-bottom: 0.5rem;
    padding: 0.5rem;
    border-radius: 4px;
    background-color: var(--background-secondary);
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .result-item:hover {
    background-color: var(--background-hover);
  }

  .result-item.selected {
    background-color: var(--background-hover);
  }

  .result-line {
    font-family: monospace;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .highlight {
    background-color: var(--highlight-color);
    padding: 0 2px;
    border-radius: 2px;
  }

  .file-path {
    color: var(--text-secondary);
    font-size: 0.9em;
    margin-bottom: 0.25rem;
  }

  .line-number {
    color: var(--text-secondary);
    margin-right: 0.5rem;
  }

  .stats {
    padding: 0.5rem;
    background-color: var(--background-secondary);
    border-top: 1px solid var(--border-color);
    font-size: 0.9em;
    color: var(--text-secondary);
  }
</style> 