// Advanced answer formatter — parses structured compliance answers into rich HTML.

export function formatAnswer(text) {
  // Phase 1: Extract leading article reference as a separate header
  let header = ''
  let body = text
  const artHeaderMatch = body.match(/^(Art\.\s*\d+(?:\([^)]+\))*(?:\s*\+\s*Art\.\s*\d+(?:\([^)]+\))*)*)\s*:\s*/)
  if (artHeaderMatch) {
    header = artHeaderMatch[1]
    body = body.slice(artHeaderMatch[0].length)
  }

  // Phase 2: Split numbered steps: (1) ... (2) ... or 1. ...
  const stepRegex = /\((\d+)\)\s/g
  const hasSteps = stepRegex.test(body)

  let html = ''

  if (header) {
    html += `<div class="ans-header"><span class="ans-art">${header}</span></div>`
  }

  if (hasSteps) {
    // Split body into intro + steps
    const firstStep = body.indexOf('(1)')
    let intro = ''
    let stepsText = body
    if (firstStep > 0) {
      intro = body.slice(0, firstStep).trim()
      stepsText = body.slice(firstStep)
    }

    if (intro) {
      // Highlight inline article references in intro
      intro = highlightArticles(intro)
      html += `<div class="ans-intro">${intro}</div>`
    }

    // Parse individual steps
    const steps = []
    const parts = stepsText.split(/\((\d+)\)\s/)
    for (let i = 1; i < parts.length; i += 2) {
      const num = parts[i]
      const content = (parts[i + 1] || '').trim().replace(/\.\s*$/, '')
      if (content) {
        steps.push({ num, content: highlightArticles(content) })
      }
    }

    if (steps.length) {
      html += '<div class="ans-steps">'
      for (const s of steps) {
        html += `<div class="ans-step"><span class="step-num">${s.num}</span><span class="step-text">${s.content}</span></div>`
      }
      html += '</div>'
    }
  } else {
    // No numbered steps — just formatted text
    body = highlightArticles(body)
    html += `<div class="ans-body">${body}</div>`
  }

  return html
}

function highlightArticles(text) {
  return text.replace(/Art\.\s*(\d+)(\([^)]+\))*/g,
    '<span class="art-ref">Art. $1$2</span>')
}
