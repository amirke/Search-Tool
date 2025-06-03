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
    fileFilter?: string;
    highlightColor?: string;
}

export interface SearchResult {
    files: SearchFile[];
    error?: string;
    scannedFiles?: number;
    filesWithMatches?: number;
    totalMatches?: number;
    durationMs?: number;
    stats?: SearchStats;
} 

export interface SearchStats {
    total_matches: number;
    matched_lines: number;
    files_searched: number;
    search_time_ms: number;
    total_time_ms: number;
}

export interface FileMatch {
    file: string;
    line: number;
    content: string;
}
