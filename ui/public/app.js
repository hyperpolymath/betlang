// SPDX-License-Identifier: PMPL-1.0-or-later
// Betlang Playground - Main Application
// Plain JavaScript implementation (no ReScript/AffineScript dependency)

// ============================================================================
// Betlang Evaluation (Simulated)
// ============================================================================

/**
 * Simulate Betlang's bet construct
 * In real Betlang: bet { "A", "B", "C" } returns a ternary probabilistic value
 * Here we simulate it with weighted random selection
 */
function bet(options, weights = null) {
  if (weights === null) {
    // Equal probability
    const index = Math.floor(Math.random() * options.length);
    return options[index];
  } else {
    // Weighted probability
    const totalWeight = weights.reduce((sum, w) => sum + w, 0);
    const r = Math.random() * totalWeight;
    let cumulative = 0;
    for (let i = 0; i < options.length; i++) {
      cumulative += weights[i];
      if (r < cumulative) {
        return options[i];
      }
    }
    return options[options.length - 1];
  }
}

/**
 * Simulate weighted bet: bet/weighted { "high" @ 0.7, "medium" @ 0.2, "low" @ 0.1 }
 */
function weightedBet(obj) {
  const options = [];
  const weights = [];
  for (const [key, value] of Object.entries(obj)) {
    if (typeof value === 'object' && value !== null && '@' in value) {
      options.push(key);
      weights.push(value['@']);
    } else {
      options.push(key);
      weights.push(value);
    }
  }
  return bet(options, weights);
}

/**
 * Parse Betlang-like code and evaluate
 * This is a simple parser for demonstration purposes
 */
function evaluateBetlangCode(code) {
  const results = [];
  const sampleCount = 1000;
  
  // Simple pattern matching for bet expressions
  const betMatch = code.match(/bet\s*\{[^}]*\}/);
  const weightedBetMatch = code.match(/bet\/weighted\s*\{[^}]*\}/);
  
  if (weightedBetMatch) {
    const content = weightedBetMatch[0].replace('bet/weighted', '').trim();
    // Parse { "a" @ 0.5, "b" @ 0.5 }
    const items = content.slice(1, -1).split(',').map(s => s.trim());
    const options = [];
    const weights = [];
    for (const item of items) {
      const match = item.match(/"([^"]+)"\s*@\s*([\d.]+)/);
      if (match) {
        options.push(match[1]);
        weights.push(parseFloat(match[2]));
      }
    }
    // Run multiple samples
    const counts = new Map();
    for (let i = 0; i < sampleCount; i++) {
      const result = bet(options, weights);
      counts.set(result, (counts.get(result) || 0) + 1);
    }
    return { type: 'weighted-bet', counts, sampleCount, options };
  } else if (betMatch) {
    const content = betMatch[0].replace('bet', '').trim();
    // Parse { "a", "b", "c" }
    const options = content.slice(1, -1).split(',').map(s => s.trim().replace(/[""]/g, ''));
    // Run multiple samples
    const counts = new Map();
    for (let i = 0; i < sampleCount; i++) {
      const result = bet(options);
      counts.set(result, (counts.get(result) || 0) + 1);
    }
    return { type: 'bet', counts, sampleCount, options };
  }
  
  // If no bet found, just return the code as-is
  return { type: 'raw', value: code };
}

// ============================================================================
// Examples
// ============================================================================

const examples = {
  'Ternary Choice': `// Basic ternary choice
// Each option has equal probability (1/3)

let outcome = bet { "A", "B", "C" }
outcome`,
  
  'Weighted Probability': `// Weighted bet - options have different probabilities
// @ specifies the weight/probability

let weighted = bet/weighted { 
  "high" @ 0.7,   // 70% chance
  "medium" @ 0.2, // 20% chance  
  "low" @ 0.1     // 10% chance
}
weighted`,
  
  'Nested Bets': `// Nested bets (hierarchical probability)
// First bet determines which sub-bet to evaluate

let nested = bet { 
  "group1", 
  "group2", 
  bet { "a", "b", "c" }
}
nested`,
  
  'Parallel Trials': `// Multiple independent bets

let trial1 = bet { "success", "failure" }
let trial2 = bet { "success", "failure" }
let trial3 = bet { "success", "failure" }

// Count successes
let successes = [trial1, trial2, trial3].filter(x => x === "success").length
successes`,
  
  'Monte Carlo Estimation': `// Monte Carlo estimation of probability
// Run many trials and count outcomes

let trials = 1000
let successes = 0

for i in 1..trials {
  let result = bet { "hit", "miss" }
  if result == "hit" { successes = successes + 1 }
}

let probability = successes / trials
probability`,
  
  'Pattern Matching': `// Pattern matching on bet results

let result = bet { "red", "green", "blue" }

match result {
  "red" -> "Stop"
  "green" -> "Go"
  "blue" -> "Caution"
}`,
  
  'Hierarchical Probability': `// Multi-level probability structure

let level1 = bet/weighted { 
  "path_a" @ 0.6,
  "path_b" @ 0.4
}

match level1 {
  "path_a" -> bet { "a1", "a2", "a3" },
  "path_b" -> bet/weighted { "b1" @ 0.5, "b2" @ 0.5 }
}`,
  
  'Probability Estimation': `// Estimate probability distribution

let samples = 5000
let results = []

for i in 1..samples {
  let outcome = bet { "option_1", "option_2", "option_3" }
  results.push(outcome)
}

// Count occurrences
let counts = {}
for r in results {
  counts[r] = (counts[r] || 0) + 1
}
counts`
};

// ============================================================================
// UI State
// ============================================================================

const state = {
  code: examples['Ternary Choice'],
  currentExample: 'Ternary Choice',
  result: null,
  sampleCount: 1000,
  running: false
};

// ============================================================================
// DOM Elements
// ============================================================================

const elements = {
  editor: null,
  runBtn: null,
  output: null,
  visualization: null,
  exampleSelector: null,
  sampleCount: null,
  shareBtn: null,
  modal: null
};

// ============================================================================
// Visualization
// ============================================================================

function createHistogram(data, sampleCount) {
  if (!data || data.type !== 'bet' && data.type !== 'weighted-bet') {
    return '<p class="no-data">Run code to see visualization</p>';
  }
  
  const maxCount = Math.max(...Array.from(data.counts.values()));
  const maxBarWidth = 300;
  const barHeight = 30;
  const spacing = 20;
  
  let html = '<svg width="400" height="' + ((data.options.length * (barHeight + spacing)) + 20) + '" class="histogram">';
  
  let y = 10;
  for (const [option, count] of data.counts.entries()) {
    const percentage = (count / sampleCount * 100).toFixed(1);
    const barWidth = (count / maxCount * maxBarWidth);
    
    html += `<g transform="translate(10, ${y})">`;
    html += `<rect width="${barWidth}" height="${barHeight}" fill="#7aa2f7" rx="4" ry="4"/>`;
    html += `<text x="${barWidth + 10}" y="${barHeight / 2 + 5}" fill="white" font-size="14">${option} (${percentage}%)</text>`;
    html += `</g>`;
    y += barHeight + spacing;
  }
  
  html += '</svg>';
  return html;
}

function createBetTree(code) {
  // Simple tree visualization for nested bets
  const hasNested = code.includes('bet {') && code.match(/bet\s*\{[^}]*bet\s*\{[^}]*\}[^}]*\}/);
  if (!hasNested) {
    return '';
  }
  
  let html = '<svg width="400" height="200" class="bet-tree">';
  html += '<circle cx="200" cy="50" r="30" fill="#7aa2f7" stroke="#1e1e2e" stroke-width="2"/>';
  html += '<text x="200" y="55" text-anchor="middle" fill="white" font-size="14">bet</text>';
  html += '<line x1="200" y1="80" x2="150" y2="130" stroke="#7aa2f7" stroke-width="2"/>';
  html += '<line x1="200" y1="80" x2="250" y2="130" stroke="#7aa2f7" stroke-width="2"/>';
  html += '<circle cx="150" cy="130" r="20" fill="#7aa2f7" opacity="0.8"/>';
  html += '<circle cx="250" cy="130" r="20" fill="#7aa2f7" opacity="0.8"/>';
  html += '<text x="150" y="135" text-anchor="middle" fill="white" font-size="12">A</text>';
  html += '<text x="250" y="135" text-anchor="middle" fill="white" font-size="12">B</text>';
  html += '</svg>';
  return html;
}

// ============================================================================
// Sharing
// ============================================================================

function generateShareURL() {
  const encodedCode = encodeURIComponent(state.code);
  return `${window.location.origin}${window.location.pathname}#code=${encodedCode}`;
}

function generateQRCode() {
  const url = generateShareURL();
  // Simple QR code SVG (placeholder - in production use a proper library)
  return `
<svg width="200" height="200" class="qr-code">
<rect width="200" height="200" fill="white"/>
<text x="100" y="90" text-anchor="middle" font-size="12" fill="#1e1e2e">QR Code</text>
<text x="100" y="110" text-anchor="middle" font-size="10" fill="#666">URL: ${encodeURIComponent(generateShareURL())}</text>
<text x="100" y="130" text-anchor="middle" font-size="10" fill="#666">(Install QR library)</text>
</svg>
`;
}

function copyToClipboard(text) {
  navigator.clipboard.writeText(text).then(() => {
    showNotification('Copied to clipboard!');
  }).catch(() => {
    // Fallback
    const el = document.createElement('textarea');
    el.value = text;
    document.body.appendChild(el);
    el.select();
    document.execCommand('copy');
    document.body.removeChild(el);
    showNotification('Copied to clipboard!');
  });
}

function showNotification(message) {
  const notification = document.createElement('div');
  notification.className = 'notification';
  notification.textContent = message;
  document.body.appendChild(notification);
  setTimeout(() => notification.remove(), 3000);
}

// ============================================================================
// Social Sharing
// ============================================================================

function shareToTwitter() {
  const url = generateShareURL();
  const text = encodeURIComponent('Check out this Betlang code: ');
  window.open(`https://twitter.com/intent/tweet?text=${text}&url=${url}`, '_blank');
}

function shareToMastodon() {
  const url = generateShareURL();
  const text = encodeURIComponent('Exploring Betlang probabilistic programming');
  window.open(`https://mastodon.social/share?text=${text} ${url}`, '_blank');
}

function shareToReddit() {
  const url = generateShareURL();
  const title = encodeURIComponent('Betlang Playground');
  window.open(`https://www.reddit.com/submit?url=${url}&title=${title}`, '_blank');
}

function shareToLinkedIn() {
  const url = generateShareURL();
  window.open(`https://www.linkedin.com/shareArticle?mini=true&url=${url}`, '_blank');
}

function shareToFacebook() {
  const url = generateShareURL();
  window.open(`https://www.facebook.com/sharer/sharer.php?u=${url}`, '_blank');
}

function shareToBluesky() {
  const url = generateShareURL();
  const text = encodeURIComponent('Check out this Betlang code');
  window.open(`https://bsky.app/intent/compose?text=${text} ${url}`, '_blank');
}

function shareToEmail() {
  const url = generateShareURL();
  window.location.href = `mailto:?subject=Betlang Playground&body=${url}`;
}

// ============================================================================
// Load from URL
// ============================================================================

function loadFromURL() {
  const hash = window.location.hash.substring(1);
  const params = new URLSearchParams(hash);
  if (params.has('code')) {
    const code = decodeURIComponent(params.get('code'));
    state.code = code;
    // Try to match with an example
    for (const [name, exampleCode] of Object.entries(examples)) {
      if (exampleCode === code) {
        state.currentExample = name;
        break;
      }
    }
  }
}

// ============================================================================
// Run Code
// ============================================================================

function runCode() {
  state.running = true;
  elements.runBtn.disabled = true;
  elements.runBtn.textContent = 'Running...';
  
  // Simulate async execution
  setTimeout(() => {
    try {
      const result = evaluateBetlangCode(state.code);
      state.result = result;
      renderOutput();
      renderVisualization();
    } catch (e) {
      elements.output.innerHTML = `<div class="error">Error: ${e.message}</div>`;
      elements.visualization.innerHTML = '';
    }
    state.running = false;
    elements.runBtn.disabled = false;
    elements.runBtn.textContent = 'Run';
  }, 100);
}

// ============================================================================
// Render Functions
// ============================================================================

function renderOutput() {
  if (!state.result) {
    elements.output.innerHTML = '<p class="placeholder">Run code to see output</p>';
    return;
  }
  
  if (state.result.type === 'bet' || state.result.type === 'weighted-bet') {
    let html = '<div class="result-header">Sample Results (' + state.sampleCount + ' runs)</div>';
    html += '<table class="result-table">';
    html += '<tr><th>Option</th><th>Count</th><th>Probability</th></tr>';
    for (const [option, count] of state.result.counts.entries()) {
      const percentage = (count / state.sampleCount * 100).toFixed(2);
      html += `<tr><td>${option}</td><td>${count}</td><td>${percentage}%</td></tr>`;
    }
    html += '</table>';
    elements.output.innerHTML = html;
  } else {
    elements.output.innerHTML = '<pre class="raw-output">' + JSON.stringify(state.result.value, null, 2) + '</pre>';
  }
}

function renderVisualization() {
  if (!state.result) {
    elements.visualization.innerHTML = '<p class="placeholder">Run code to see visualization</p>';
    return;
  }
  
  if (state.result.type === 'bet' || state.result.type === 'weighted-bet') {
    elements.visualization.innerHTML = createHistogram(state.result, state.sampleCount);
  } else {
    elements.visualization.innerHTML = createBetTree(state.code);
  }
}

function updateSampleCount() {
  state.sampleCount = parseInt(elements.sampleCount.value) || 1000;
  if (state.result) {
    // Re-run with new sample count
    runCode();
  }
}

function selectExample() {
  const selected = elements.exampleSelector.value;
  state.code = examples[selected];
  state.currentExample = selected;
  elements.editor.value = state.code;
  elements.output.innerHTML = '<p class="placeholder">Run code to see output</p>';
  elements.visualization.innerHTML = '<p class="placeholder">Run code to see visualization</p>';
  state.result = null;
  
  // Update URL
  window.history.pushState({}, '', generateShareURL());
}

// ============================================================================
// Modal
// ============================================================================

function showModal() {
  elements.modal.style.display = 'block';
  document.getElementById('share-url').value = generateShareURL();
  document.getElementById('qr-code').innerHTML = generateQRCode();
}

function hideModal() {
  elements.modal.style.display = 'none';
}

// ============================================================================
// Initialize
// ============================================================================

function init() {
  // Get DOM elements
  elements.editor = document.getElementById('editor');
  elements.runBtn = document.getElementById('run-btn');
  elements.output = document.getElementById('output');
  elements.visualization = document.getElementById('visualization');
  elements.exampleSelector = document.getElementById('example-selector');
  elements.sampleCount = document.getElementById('sample-count');
  elements.shareBtn = document.getElementById('share-btn');
  elements.modal = document.getElementById('share-modal');
  
  // Set up editor
  elements.editor.value = state.code;
  
  // Populate example selector
  for (const name of Object.keys(examples)) {
    const option = document.createElement('option');
    option.value = name;
    option.textContent = name;
    elements.exampleSelector.appendChild(option);
  }
  elements.exampleSelector.value = state.currentExample;
  
  // Set sample count
  elements.sampleCount.value = state.sampleCount;
  
  // Load from URL
  loadFromURL();
  
  // Event listeners
  elements.runBtn.addEventListener('click', runCode);
  elements.exampleSelector.addEventListener('change', selectExample);
  elements.sampleCount.addEventListener('change', updateSampleCount);
  elements.shareBtn.addEventListener('click', showModal);
  
  document.querySelector('.modal-close').addEventListener('click', hideModal);
  document.querySelector('.copy-btn').addEventListener('click', () => {
    copyToClipboard(generateShareURL());
  });
  
  // Social buttons
  document.getElementById('share-twitter').addEventListener('click', shareToTwitter);
  document.getElementById('share-mastodon').addEventListener('click', shareToMastodon);
  document.getElementById('share-reddit').addEventListener('click', shareToReddit);
  document.getElementById('share-linkedin').addEventListener('click', shareToLinkedIn);
  document.getElementById('share-facebook').addEventListener('click', shareToFacebook);
  document.getElementById('share-bluesky').addEventListener('click', shareToBluesky);
  document.getElementById('share-email').addEventListener('click', shareToEmail);
  
  // Close modal on outside click
  window.addEventListener('click', (e) => {
    if (e.target === elements.modal) {
      hideModal();
    }
  });
  
  // Keyboard shortcuts
  window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key === 'Enter') {
      runCode();
    }
    if (e.key === 'Escape') {
      hideModal();
    }
  });
  
  // Initial render
  renderOutput();
  renderVisualization();
  
  console.log('Betlang Playground initialized');
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
