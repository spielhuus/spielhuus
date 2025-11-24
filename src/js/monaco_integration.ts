import * as monaco from 'monaco-editor';

import 'monaco-editor/esm/vs/editor/editor.worker';
import 'monaco-editor/esm/vs/language/json/json.worker';

export interface EditorError {
  line: number;
  column: number;
  length: number;
  message: string;
}

function getBasePath() {
  // @ts-ignore
  const baseUrl = window.siteBaseUrl;
  if (baseUrl) {
    return `${baseUrl}/monaco-editor/vs`;
  } else {
    return '/monaco-editor/vs';
  }
}

self.MonacoEnvironment = {
  getWorkerUrl: function(_moduleId: any, label: string) {
    const path = getBasePath();
    if (label === 'json') {
      return `${path}/language/json/json.worker.js`;
    }
    if (label === 'css' || label === 'scss' || label === 'less') {
      return `${path}/language/css/css.worker.js`;
    }
    if (label === 'html' || label === 'handlebars' || label === 'razor') {
      return `${path}/language/html/html.worker.js`;
    }
    if (label === 'typescript' || label === 'javascript') {
      return `${path}/language/typescript/ts.worker.js`;
    }
    return `${path}/editor/editor.worker.js`;
  }
};

export class CodeEditor {
  editor: monaco.editor.IStandaloneCodeEditor;
  private decorations: monaco.editor.IEditorDecorationsCollection;

  constructor(containerId: string, initialValue: string, language: string = 'javascript') {
    const container = document.getElementById(containerId);
    if (!container) throw new Error(`Container #${containerId} not found`);

    const currentTheme = document.documentElement.getAttribute('data-theme') === 'dark' ? 'vs-dark' : 'vs';

    this.editor = monaco.editor.create(container, {
      value: initialValue,
      language: language,
      theme: currentTheme,
      automaticLayout: true,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      fontFamily: "JetBrainsMono-Regular, 'Roboto Mono', monospace",
      fontSize: 14,
      fontLigatures: true,
      scrollBeyondLastColumn: 20,
    });
    // Initialize empty collection
    this.decorations = this.editor.createDecorationsCollection([]);


    // @ts-ignore
    if (window.themeController) {
      // @ts-ignore
      window.themeController.subscribe((newTheme: string) => {
        const monacoTheme = newTheme === 'dark' ? 'vs-dark' : 'vs';
        monaco.editor.setTheme(monacoTheme);
      });
    }

    document.fonts.ready.then(() => {
      monaco.editor.remeasureFonts();
    });
  }

  setMarkers(errors: EditorError[]) {
    const model = this.editor.getModel();
    if (!model) return;

    const markers: monaco.editor.IMarkerData[] = errors.map(e => {
      const startLine = e.line > 0 ? e.line : 1;
      const startCol = e.column > 0 ? e.column : 1;
      const length = e.length > 0 ? e.length : 1;

      return {
        severity: monaco.MarkerSeverity.Error,
        message: e.message,
        startLineNumber: startLine,
        startColumn: startCol,
        endLineNumber: startLine,
        endColumn: startCol + length,
      };
    });

    monaco.editor.setModelMarkers(model, 'wgpu-owner', markers);

    const newDecorations: monaco.editor.IModelDeltaDecoration[] = [];

    for (const e of errors) {
      const startLine = e.line > 0 ? e.line : 1;

      if (startLine > model.getLineCount()) continue;

      const lineContent = model.getLineContent(startLine);
      const endOfLineCol = lineContent.length + 1;

      let safeMessage = e.message.split('\n')[0];
      safeMessage = safeMessage
        .replace(/\\/g, '\\\\')
        .replace(/"/g, '\\"')
        .replace(/'/g, "\\'");
      newDecorations.push({
        range: new monaco.Range(startLine, endOfLineCol, startLine, endOfLineCol),
        options: {
          isWholeLine: false,
          after: {
            content: `    ⛔ ${safeMessage}`,
            attachedData: e.message,
            inlineClassName: 'inline-error-decoration',
          },
          stickiness: monaco.editor.TrackedRangeStickiness.NeverGrowsWhenTypingAtEdges
        }
      });
    }

    this.decorations.set(newDecorations);
  }

  getValue(): string {
    return this.editor.getValue();
  }
}
