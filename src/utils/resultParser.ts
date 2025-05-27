import type { SearchFile } from '../types/search';

export function parseResults(text: string): SearchFile[] {
    try {
        const fileMap = new Map<string, SearchFile>();
        
        const lines = text.split('\n');
        
        lines.forEach((line) => {
            if (line.startsWith('<line') && line.endsWith('</line>')) {
                try {
                    const fileMatch = line.match(/file="([^"]+)"/);
                    const numMatch = line.match(/num="(\d+)"/);
                    const content = line.slice(line.indexOf('>') + 1, -7);
                    
                    if (fileMatch && numMatch) {
                        const fileName = fileMatch[1];
                        const lineNum = numMatch[1];
                        
                        if (!fileMap.has(fileName)) {
                            fileMap.set(fileName, { name: fileName, lines: [] });
                        }
                        fileMap.get(fileName)?.lines.push({ num: lineNum, content });
                    }
                } catch (e) {
                    // Silently skip malformed lines
                }
            }
        });

        // Convert map to array and sort
        const sortedFiles = Array.from(fileMap.values());
        sortedFiles.sort((a, b) => a.name.localeCompare(b.name));
        return sortedFiles;
    } catch (e) {
        throw new Error('Failed to parse search results');
    }
} 