import "@testing-library/jest-dom/vitest";

// jsdom has no canvas; charts detect this and render their text summary only.
HTMLCanvasElement.prototype.getContext = () => null;

// jsdom has no matchMedia; uPlot and the theme listener only need the shape.
window.matchMedia = (query: string) =>
  ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: () => undefined,
    removeEventListener: () => undefined,
    dispatchEvent: () => false,
  }) as unknown as MediaQueryList;
