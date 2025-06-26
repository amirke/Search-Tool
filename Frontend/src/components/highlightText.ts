import type { Action } from 'svelte/action';

// Function to highlight text with the selected color
function highlightText(text: string, query: string, color: string): string {
  if (!query) return text;
  if (query === ':') {
    // Special case: highlight all colons
    return text.replace(/(:)/g, `<span style="background-color: ${color}">$1</span>`);
  }
  // Escape special regex characters
  const escapedQuery = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const regex = new RegExp(`(${escapedQuery})`, 'g');
  return text.replace(regex, `<span style="background-color: ${color}">$1</span>`);
}

// Action to highlight text
export const highlightTextAction: Action<HTMLElement, { text: string; query: string; color: string }> = (node, { text, query, color }) => {
  const update = () => {
    node.innerHTML = highlightText(text, query, color);
  };

  update();

  return {
    update: ({ text, query, color }) => {
      node.innerHTML = highlightText(text, query, color);
    }
  };
}; 