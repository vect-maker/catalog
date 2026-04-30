const escapeRegExp = (string: string) => {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

export const getHighlightedTokens = (text: string, query: string) => {
  if (!query.trim()) return [{ text, isMatch: false }];

  const escapedQuery = escapeRegExp(query);
  const regex = new RegExp(`(${escapedQuery})`, 'gi'); 
  
  return text.split(regex).map(part => ({
    text: part,
    isMatch: part.toLowerCase() === query.toLowerCase()
  })).filter(part => part.text.length > 0); 
};

export const getMatchSnippet = (text: string | undefined | null, query: string, padding: number = 30): string | null => {
  if (!text || !query.trim()) return null;
  
  const lowerText = text.toLowerCase();
  const lowerQuery = query.toLowerCase();
  const matchIndex = lowerText.indexOf(lowerQuery);
  
  if (matchIndex === -1) return null;

  const start = Math.max(0, matchIndex - padding);
  const end = Math.min(text.length, matchIndex + query.length + padding);
  
  let snippet = text.substring(start, end);
  if (start > 0) snippet = '...' + snippet;
  if (end < text.length) snippet = snippet + '...';
  
  return snippet;
};