// File: resultParser.ts
// Purpose: Parses raw output from ripgrep (and backend) into structured results and stats for the frontend.
// Used by: SearchResults.svelte, searchService.ts
// Special: Expects stats block in output, parses both results and stats.
import type { SearchFile } from '../types/search';

interface SearchStats {
    scanned: number;
    files: number;
    matches: number;
    duration: number;
}

interface ParsedResult {
    file: string;
    line: number;
    content: string;
}

interface ParsedOutput {
    results: ParsedResult[];
    stats: SearchStats;
}

export function parseResults(rawResults: string): ParsedOutput {
    // Parses the raw string output from ripgrep and backend.
    // Returns: { results: ParsedResult[], stats: SearchStats }
    // Used by: SearchResults.svelte, searchService.ts
    // Special: Looks for <stats>...</stats> block for stats, parses result lines as file:line:content
    console.log('Parsing raw results:', rawResults);
    
    // Split the raw results into lines for processing
    const lines = rawResults.split('\n');
    const results: ParsedResult[] = [];
    let stats: SearchStats = {
        scanned: 0,
        files: 0,
        matches: 0,
        duration: 0
    };

    // Find the stats section in the output, if present
    const statsStartIndex = lines.findIndex(line => line.includes('<stats>'));
    const statsEndIndex = lines.findIndex(line => line.includes('</stats>'));
    
    if (statsStartIndex !== -1 && statsEndIndex !== -1) {
        // Extract and parse stats lines
        const statsLines = lines.slice(statsStartIndex + 1, statsEndIndex);
        console.log('Stats block lines:', statsLines);
        statsLines.forEach(line => {
            console.log('Parsing stats line:', line);
            if (line.includes('scanned_files:')) {
                stats.scanned = parseInt(line.split(':')[1].trim());
            } else if (line.includes('files_with_matches:')) {
                stats.files = parseInt(line.split(':')[1].trim());
            } else if (line.includes('total_matches:')) {
                stats.matches = parseInt(line.split(':')[1].trim());
            } else if (line.includes('duration_ms:')) {
                stats.duration = parseInt(line.split(':')[1].trim());
            }
        });
    } else {
        // Warn if stats block is missing
        console.warn('Stats block not found in results!');
    }

    // Parse each result line (format: file:line:content)
    lines.forEach(line => {
        // Skip empty lines and stats section
        if (!line.trim() || line.includes('<stats>') || line.includes('</stats>')) {
            return;
        }

        // Try to parse the line as a result
        const match = line.match(/^(.+?):(\d+):(.+)$/);
        if (match) {
            const [_, file, lineNum, content] = match;
            results.push({
                file: file.trim(),
                line: parseInt(lineNum),
                content: content.trim()
            });
        }
    });

    console.log('Parsed stats:', stats);
    console.log('Found result lines:', results.length);

    // Return parsed results and stats for use in frontend
    return {
        results,
        stats
    };
} 