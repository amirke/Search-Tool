import { writable } from 'svelte/store';

export const isSearchingfromSearchservice = writable(false);

export const lastDuration = writable(0);      // holds the most recent duration in ms
export const lastTimestamp = writable('');    // holds the formatted time

export const expandResults = writable(true);
