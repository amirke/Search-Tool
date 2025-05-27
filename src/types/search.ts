export interface SearchLine {
    num: string;
    content: string;
}

export interface SearchFile {
    name: string;
    lines: SearchLine[];
}

export interface SearchParams {
    query: string;
    path: string;
    highlightColor?: string;
}

export interface SearchResult {
    files: SearchFile[];
    error?: string;
    scannedFiles?: number;
    filesWithMatches?: number;
    totalMatches?: number;
    durationMs?: number;
} 