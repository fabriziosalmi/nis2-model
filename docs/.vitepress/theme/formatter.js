// Answer formatting — parses structured text into rich HTML.

export function formatAnswer(text) {
  let html = text
  // Article references → badges
  html = html.replace(/Art\.\s*(\d+)(\([^)]+\))*/g,
    '<span class="art-badge">Art. $1$2</span>')
  // Numbered items: (1) … (2) …
  html = html.replace(/\((\d+)\)\s/g,
    '<br><span class="step-num">$1</span> ')
  // Numbered items: 1. … 2. …
  html = html.replace(/(\d+)\.\s(?=[A-Z])/g,
    '<br><span class="step-num">$1</span> ')
  return html
}
