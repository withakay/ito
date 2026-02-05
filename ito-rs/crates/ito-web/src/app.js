// State
let editor = null, currentPath = null, originalContent = null, currentLanguage = null, previewMode = false;
let viewMode = 'files';
let currentTemplatePath = null, currentTemplateSource = null;
let term = null, termSocket = null, fitAddon = null;

const langToMode = {
  'rust': 'rust', 'javascript': 'javascript', 'typescript': 'javascript', 'python': 'python',
  'go': 'go', 'markdown': 'markdown', 'json': { name: 'javascript', json: true },
  'yaml': 'yaml', 'toml': 'toml', 'html': 'htmlmixed', 'css': 'css', 'sql': 'sql', 'shell': 'shell', 'xml': 'xml', 'text': 'text'
};

const fileIcons = { 'rs': '', 'js': '', 'ts': '', 'py': '', 'go': '', 'md': '', 'json': '', 'yaml': '', 'yml': '', 'toml': '', 'sh': '', 'html': '', 'css': '', 'default': '', 'dir': '' };

function getIcon(name, isDir) {
  if (isDir) return fileIcons.dir;
  return fileIcons[name.split('.').pop()?.toLowerCase()] || fileIcons.default;
}

async function listDir(path = '') {
  const res = await fetch(path ? `/api/list/${path}` : '/api/list');
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

async function listTemplates() {
  const res = await fetch('/api/templates/list');
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

async function readTemplateSource(path) {
  const res = await fetch(`/api/templates/source?path=${encodeURIComponent(path)}`);
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

async function renderTemplate(path, ctx) {
  const res = await fetch(`/api/templates/render?path=${encodeURIComponent(path)}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(ctx)
  });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

async function readFile(path) {
  const res = await fetch(`/api/file/${path}`);
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

async function saveFile(path, content) {
  const res = await fetch(`/api/file/${path}`, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ content }) });
  if (!res.ok) throw new Error(await res.text());
  return res.json();
}

function updateBreadcrumb(path) {
  const bc = document.getElementById('breadcrumb');
  const parts = path ? path.split('/') : [];
  let html = '<span data-path="">~</span>';
  let accumulated = '';
  for (const part of parts) { accumulated += (accumulated ? '/' : '') + part; html += ` / <span data-path="${accumulated}">${escapeHtml(part)}</span>`; }
  bc.innerHTML = html;
  bc.querySelectorAll('span').forEach(span => { span.onclick = () => navigateDir(span.dataset.path); });
}

async function navigateDir(path) {
  try { const data = await listDir(path); renderTree(data.entries, path); updateBreadcrumb(path); }
  catch (e) { console.error('Failed to navigate:', e); }
}

async function navigateTemplates() {
  try {
    const data = await listTemplates();
    renderTemplatesTree(data.templates);
    const bc = document.getElementById('breadcrumb');
    bc.innerHTML = '<span data-path="">Templates</span>';
  } catch (e) {
    console.error('Failed to list templates:', e);
  }
}

function renderTree(entries, parentPath) {
  const tree = document.getElementById('file-tree');
  tree.innerHTML = '';
  if (parentPath) {
    const item = document.createElement('div');
    item.className = 'tree-item';
    item.innerHTML = '<span class="icon"></span><span class="name">..</span>';
    item.onclick = () => navigateDir(parentPath.split('/').slice(0, -1).join('/'));
    tree.appendChild(item);
  }
  for (const entry of entries) {
    const item = document.createElement('div');
    item.className = 'tree-item';
    if (entry.path === currentPath) item.classList.add('active');
    item.innerHTML = `<span class="icon">${getIcon(entry.name, entry.is_dir)}</span><span class="name">${escapeHtml(entry.name)}</span>`;
    item.onclick = () => entry.is_dir ? navigateDir(entry.path) : openFile(entry.path);
    tree.appendChild(item);
  }
}

function renderTemplatesTree(templates) {
  const tree = document.getElementById('file-tree');
  tree.innerHTML = '';
  for (const t of templates) {
    const item = document.createElement('div');
    item.className = 'tree-item';
    if (t.path === currentTemplatePath) item.classList.add('active');
    item.innerHTML = `<span class="icon"></span><span class="name">${escapeHtml(t.path)}</span>`;
    item.onclick = () => openTemplate(t.path);
    tree.appendChild(item);
  }
}

async function openFile(path) {
  try {
    if (viewMode !== 'files') return;
    if (editor && hasChanges() && !confirm('Discard unsaved changes?')) return;
    document.getElementById('sidebar').classList.remove('open');
    document.getElementById('sidebar-overlay').classList.remove('visible');
    const data = await readFile(path);
    currentPath = path; originalContent = data.content; currentLanguage = data.language; previewMode = false;
    document.getElementById('toolbar').style.display = 'flex';
    document.getElementById('filename').textContent = path.split('/').pop();
    document.getElementById('mobile-filename').textContent = path.split('/').pop();
    document.getElementById('status').textContent = '';
    document.getElementById('preview-btn').style.display = data.language === 'markdown' ? 'inline-flex' : 'none';
    document.getElementById('preview-btn').classList.remove('active');
    document.getElementById('preview-container').classList.remove('visible');
    document.getElementById('editor-container').style.display = 'block';
    document.querySelectorAll('.tree-item').forEach(el => el.classList.toggle('active', el.querySelector('.name')?.textContent === path.split('/').pop()));
    const container = document.getElementById('editor-container');
    container.innerHTML = '';
    const textarea = document.createElement('textarea');
    container.appendChild(textarea);
    editor = CodeMirror.fromTextArea(textarea, { mode: langToMode[data.language] || 'text', theme: 'dracula', lineNumbers: true, lineWrapping: true, tabSize: 2, autofocus: true, extraKeys: { 'Cmd-S': doSave, 'Ctrl-S': doSave, 'Cmd-P': togglePreview, 'Ctrl-P': togglePreview } });
    editor.setValue(data.content);
    editor.on('change', updateSaveState);
    updateSaveState();
  } catch (e) { console.error('Failed to open:', e); setStatus('error', e.message); }
}

async function openTemplate(path) {
  try {
    if (viewMode !== 'templates') return;
    document.getElementById('sidebar').classList.remove('open');
    document.getElementById('sidebar-overlay').classList.remove('visible');

    const src = await readTemplateSource(path);
    currentTemplatePath = path;
    currentTemplateSource = src.content;

    document.getElementById('toolbar').style.display = 'none';
    const container = document.getElementById('editor-container');
    container.innerHTML = `
      <div class="template-view">
        <div class="template-header">
          <div class="template-title">Template: ${escapeHtml(path)}</div>
          <button class="btn btn-secondary" id="template-back">Back</button>
          <button class="btn btn-secondary" id="template-md-toggle" title="Toggle Markdown preview">MD</button>
          <button class="btn btn-primary" id="template-render">Render</button>
        </div>
        <div class="template-grid">
          <div class="template-panel">
            <h3>Context (JSON)</h3>
            <textarea id="template-context" spellcheck="false">{\n  "name": "world"\n}</textarea>
          </div>
          <div class="template-panel">
            <h3>Template Source</h3>
            <pre id="template-source"></pre>
          </div>
          <div class="template-panel" id="template-output-panel">
            <h3>Rendered Output</h3>
            <pre id="template-output"></pre>
            <div class="template-output-md" id="template-output-md"></div>
          </div>
        </div>
      </div>
    `;

    document.getElementById('template-source').textContent = currentTemplateSource;
    document.getElementById('template-back').onclick = () => { currentTemplatePath = null; currentTemplateSource = null; container.innerHTML = ''; navigateTemplates(); };

    const mdToggle = document.getElementById('template-md-toggle');
    mdToggle.onclick = () => {
      const md = document.getElementById('template-output-md');
      const pre = document.getElementById('template-output');
      const on = md.classList.toggle('visible');
      pre.style.display = on ? 'none' : 'block';
      if (on) md.innerHTML = marked.parse(pre.textContent || '');
    };

    document.getElementById('template-render').onclick = async () => {
      const ctxText = document.getElementById('template-context').value;
      let ctx;
      try {
        ctx = JSON.parse(ctxText);
      } catch (e) {
        setStatus('error', `Invalid JSON: ${e.message}`);
        return;
      }

      try {
        setStatus('', '');
        const out = await renderTemplate(path, ctx);
        const pre = document.getElementById('template-output');
        pre.textContent = out.output;
        const md = document.getElementById('template-output-md');
        if (md.classList.contains('visible')) md.innerHTML = marked.parse(out.output);
      } catch (e) {
        setStatus('error', e.message);
      }
    };

    document.querySelectorAll('.tree-item').forEach(el => el.classList.toggle('active', el.querySelector('.name')?.textContent === path));
  } catch (e) {
    console.error('Failed to open template:', e);
    setStatus('error', e.message);
  }
}

function hasChanges() { return editor && editor.getValue() !== originalContent; }

function updateSaveState() {
  document.getElementById('save-btn').disabled = !hasChanges();
  document.getElementById('filename').classList.toggle('modified', hasChanges());
}

async function doSave() {
  if (!currentPath || !editor) return;
  const btn = document.getElementById('save-btn');
  try {
    btn.disabled = true; btn.textContent = 'Saving...';
    await saveFile(currentPath, editor.getValue());
    originalContent = editor.getValue();
    setStatus('saved', 'Saved');
    updateSaveState();
    if (previewMode) updatePreview();
  } catch (e) { setStatus('error', e.message); }
  finally { btn.innerHTML = 'Save <kbd>⌘S</kbd>'; }
}

function setStatus(type, msg) {
  const s = document.getElementById('status');
  s.textContent = msg; s.className = `status ${type}`;
  if (type === 'saved') setTimeout(() => { if (s.textContent === msg) s.textContent = ''; }, 2000);
}

function togglePreview() {
  if (currentLanguage !== 'markdown') return false;
  previewMode = !previewMode;
  document.getElementById('preview-btn').classList.toggle('active', previewMode);
  document.getElementById('editor-container').style.display = previewMode ? 'none' : 'block';
  document.getElementById('preview-container').classList.toggle('visible', previewMode);
  if (previewMode) updatePreview(); else editor?.focus();
  return false;
}

function updatePreview() {
  if (editor) document.getElementById('preview-container').innerHTML = marked.parse(editor.getValue());
}

function initTerminal() {
  fitAddon = new window.FitAddon.FitAddon();
  term = new Terminal({
    theme: { background: '#1a1b26', foreground: '#c0caf5', cursor: '#c0caf5', black: '#15161e', red: '#f7768e', green: '#9ece6a', yellow: '#e0af68', blue: '#7aa2f7', magenta: '#bb9af7', cyan: '#7dcfff', white: '#a9b1d6' },
    fontFamily: "'MesloLGS NF', 'JetBrains Mono', monospace", fontSize: 14, cursorBlink: true
  });
  term.loadAddon(fitAddon);
  term.open(document.getElementById('terminal'));
  setTimeout(() => fitAddon.fit(), 100);
  connectTerminal();
  window.addEventListener('resize', () => fitAddon.fit());
  new ResizeObserver(() => setTimeout(() => fitAddon.fit(), 50)).observe(document.getElementById('terminal-panel'));
  term.onData(data => { if (termSocket?.readyState === WebSocket.OPEN) termSocket.send(data); });
}

function connectTerminal() {
  termSocket = new WebSocket(`${location.protocol === 'https:' ? 'wss:' : 'ws:'}//${location.host}/ws/terminal`);
  termSocket.binaryType = 'arraybuffer';
  termSocket.onopen = () => { document.getElementById('terminal-indicator').classList.add('connected'); termSocket.send(JSON.stringify({ resize: { cols: term.cols, rows: term.rows } })); };
  termSocket.onmessage = (e) => term.write(e.data instanceof ArrayBuffer ? new Uint8Array(e.data) : e.data);
  termSocket.onclose = () => { document.getElementById('terminal-indicator').classList.remove('connected'); term.write('\r\n\x1b[31mDisconnected. Reconnecting...\x1b[0m\r\n'); setTimeout(connectTerminal, 2000); };
  term.onResize(({ cols, rows }) => { if (termSocket?.readyState === WebSocket.OPEN) termSocket.send(JSON.stringify({ resize: { cols, rows } })); });
}

function initTerminalResize() {
  const panel = document.getElementById('terminal-panel'), handle = document.getElementById('resize-handle'), main = document.querySelector('.main');
  let startY, startHeight;
  const resize = (y) => { panel.style.height = Math.max(100, Math.min(main.offsetHeight - 100, startHeight + startY - y)) + 'px'; };
  handle.addEventListener('mousedown', (e) => { startY = e.clientY; startHeight = panel.offsetHeight; document.body.style.cursor = 'ns-resize'; document.addEventListener('mousemove', onMove); document.addEventListener('mouseup', onUp); e.preventDefault(); });
  handle.addEventListener('touchstart', (e) => { startY = e.touches[0].clientY; startHeight = panel.offsetHeight; document.addEventListener('touchmove', onTouchMove); document.addEventListener('touchend', onTouchEnd); });
  const onMove = (e) => resize(e.clientY);
  const onTouchMove = (e) => resize(e.touches[0].clientY);
  const onUp = () => { document.body.style.cursor = ''; document.removeEventListener('mousemove', onMove); document.removeEventListener('mouseup', onUp); fitAddon?.fit(); };
  const onTouchEnd = () => { document.removeEventListener('touchmove', onTouchMove); document.removeEventListener('touchend', onTouchEnd); fitAddon?.fit(); };
  document.getElementById('terminal-toggle').onclick = () => { panel.classList.remove('maximized'); panel.classList.toggle('collapsed'); setTimeout(() => fitAddon?.fit(), 100); };
  document.getElementById('terminal-maximize').onclick = () => { panel.classList.remove('collapsed'); panel.classList.toggle('maximized'); setTimeout(() => fitAddon?.fit(), 100); };
}

function initMobile() {
  document.getElementById('menu-btn').onclick = () => { document.getElementById('sidebar').classList.add('open'); document.getElementById('sidebar-overlay').classList.add('visible'); };
  document.getElementById('sidebar-overlay').onclick = () => { document.getElementById('sidebar').classList.remove('open'); document.getElementById('sidebar-overlay').classList.remove('visible'); };
}

function setViewMode(mode) {
  viewMode = mode;
  document.getElementById('mode-files').classList.toggle('active', mode === 'files');
  document.getElementById('mode-templates').classList.toggle('active', mode === 'templates');
  currentTemplatePath = null;
  currentTemplateSource = null;
  document.getElementById('toolbar').style.display = 'none';
  document.getElementById('editor-container').innerHTML = '<div class="empty-state"><div class="icon">DIR</div><p>Select a file to edit</p><p><kbd>⌘S</kbd> Save <kbd>⌘P</kbd> Preview <kbd>⌘`</kbd> Terminal</p></div>';
  if (mode === 'files') navigateDir('');
  else navigateTemplates();
}

function escapeHtml(t) { const d = document.createElement('div'); d.textContent = t; return d.innerHTML; }

document.addEventListener('keydown', (e) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 's') { e.preventDefault(); doSave(); }
  if ((e.metaKey || e.ctrlKey) && e.key === 'p') { e.preventDefault(); togglePreview(); }
  if ((e.metaKey || e.ctrlKey) && e.key === '`') { e.preventDefault(); document.getElementById('terminal-panel').classList.toggle('collapsed'); setTimeout(() => { fitAddon?.fit(); term?.focus(); }, 100); }
});

window.onbeforeunload = () => hasChanges() ? 'Unsaved changes' : undefined;

document.getElementById('save-btn').onclick = doSave;
document.getElementById('preview-btn').onclick = togglePreview;
document.getElementById('mode-files').onclick = () => setViewMode('files');
document.getElementById('mode-templates').onclick = () => setViewMode('templates');
navigateDir('');
initTerminal();
initTerminalResize();
initMobile();
