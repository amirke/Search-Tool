import { invoke } from "@tauri-apps/api";
import type { SearchParams, SearchResult } from '../types/search';
import { parseResults } from '../utils/resultParser';

export async function browseFolder(): Promise<string | null> {
    try {
        const folder = await invoke('open_folder_dialog') as string;
        return folder || null;
    } catch (err) {
        throw new Error(`Browse failed: ${err}`);
    }
}

export async function search(params: SearchParams): Promise<SearchResult> {
    if (!params.query.trim()) {
        throw new Error('Please enter a search query');
    }
    if (!params.path.trim()) {
        throw new Error('Please select a folder to search in');
    }

    try {
        const rawResults = await invoke('search_text', {
            query: params.query.trim(),
            path: params.path.trim()
        }) as string;
        
        if (!rawResults || !rawResults.trim()) {
            return { files: [] };
        }
        
        const files = parseResults(rawResults);
        return { files };
    } catch (e) {
        if (e instanceof Error) {
            return { files: [], error: e.message };
        } else if (typeof e === 'string') {
            return { files: [], error: e };
        } else {
            return { files: [], error: 'Unknown error occurred' };
        }
    }
}

export async function readFile(path: string, lineNumber?: number): Promise<string> {
    try {
        return await invoke('read_file', { path, lineNumber }) as string;
    } catch (e) {
        throw new Error(`Failed to load file preview: ${e}`);
    }
}

export async function checkPortStatus(): Promise<void> {
    try {
        const result = await invoke('ensure_port_1420_available') as string;
        console.log('[Frontend] ' + result);
    } catch (e) {
        const msg = 'Failed to check port: ' + (e instanceof Error ? e.message : e);
        console.log('[Frontend] ' + msg);
    }
} 