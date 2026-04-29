// BM25 search engine + language detection for the NIS2 chatbot.

export function tokenize(t) {
  return t.toLowerCase()
    .replace(/[''`]/g, "'")
    .replace(/[^a-z0-9àèéìòùäöüß\s']/g, ' ')
    .split(/\s+/)
    .filter(t => t.length > 1)
}

export function isItalian(text) {
  const markers = [
    'serve','cosa','come','quali','siamo','nostri','possiamo','rientra',
    'dobbiamo','quanto','azienda','cifrat','incidente','fornitor',
    'sicurezza','backup','audit','formazione','obblig','sanzioni',
    'continuita','crittografia','rischi','gestione','entita','allegato',
    'della','delle','nella','nelle','degli','sono','essere','questo',
    'questa','ogni','deve','devo','fare','perch','anche','ancora',
    'quando','dove','settori','direttiva','notifica','prevede',
    'implementare','necessario','dipende','operativa','conformit',
    'misure','procedur','preoccup','hackerato','compromess',
    'violazione','segnalazione','adeguarsi','rispetto',
  ]
  const low = text.toLowerCase()
  return markers.some(w => low.includes(w)) || /[àèéìòù]/.test(low)
}

export function bm25Search(dataset, query, topN = 5) {
  const qT = tokenize(query)
  if (!qT.length) return []
  const N = dataset.length, k1 = 1.5, b = 0.75
  const avgDl = dataset.reduce((s, d) => s + tokenize(d.q).length, 0) / N
  const df = {}
  for (const t of qT) df[t] = dataset.filter(d => tokenize(d.q).includes(t)).length
  return dataset.map((doc, idx) => {
    const dT = tokenize(doc.q), dl = dT.length
    let score = 0
    for (const t of qT) {
      const tf = dT.filter(dt => dt === t).length
      const idf = Math.log((N - (df[t] || 0) + 0.5) / ((df[t] || 0) + 0.5) + 1)
      score += idf * (tf * (k1 + 1)) / (tf + k1 * (1 - b + b * dl / avgDl))
    }
    return { score, idx }
  }).sort((a, b) => b.score - a.score).slice(0, topN).filter(s => s.score > 0)
}

export function findFollowUps(dataset, category, lang) {
  const baseCat = category.replace(/_impl$/, '')
  const related = [category, baseCat, baseCat + '_impl']
  const candidates = dataset.filter(d => {
    const dCat = d.c.replace(/_impl$/, '')
    return related.includes(d.c) || related.includes(dCat)
  })
  const langFiltered = candidates.filter(d =>
    lang === 'it' ? isItalian(d.q) : !isItalian(d.q)
  )
  const pool = langFiltered.length > 0 ? langFiltered : candidates
  const seen = new Set()
  const result = []
  for (const d of pool) {
    if (!seen.has(d.q) && result.length < 4) {
      seen.add(d.q)
      result.push(d.q)
    }
  }
  return result
}
