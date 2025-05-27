export function getRelativePath(fullPath: string, basePath: string): string {
    if (!basePath || basePath === '.') return fullPath;
    
    // Convert both paths to forward slashes for consistent comparison
    const normalizedPath = basePath.replace(/\\/g, '/');
    const normalizedFullPath = fullPath.replace(/\\/g, '/');
    
    // If the full path starts with the search path, remove it
    if (normalizedFullPath.startsWith(normalizedPath)) {
        return normalizedFullPath.slice(normalizedPath.length).replace(/^[\/\\]/, '');
    }
    
    // If we can't make it relative, return the last part of the path
    const parts = normalizedFullPath.split('/');
    return parts[parts.length - 1];
}

export function getDisplayPath(fullPath: string, basePath: string): string {
    const relativePath = getRelativePath(fullPath, basePath);
    return relativePath.length > 50 ? '...' + relativePath.slice(-47) : relativePath;
} 