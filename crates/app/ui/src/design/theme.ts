// Light and dark follow the system; Setup can pin one. Stored per viewer.
export type ThemeChoice = "system" | "light" | "dark";
const KEY = "bigman.theme";

export function storedTheme(): ThemeChoice {
  try {
    const value = localStorage.getItem(KEY);
    return value === "light" || value === "dark" ? value : "system";
  } catch {
    return "system";
  }
}

export function applyTheme(choice: ThemeChoice): void {
  const root = document.documentElement;
  if (choice === "system") root.removeAttribute("data-theme");
  else root.setAttribute("data-theme", choice);
  try {
    if (choice === "system") localStorage.removeItem(KEY);
    else localStorage.setItem(KEY, choice);
  } catch {
    // Storage can be unavailable; the choice still applies for this session.
  }
}

/** Reads a token's computed value, for canvas charts that cannot use CSS. */
export function token(name: string, element: Element = document.documentElement): string {
  return getComputedStyle(element).getPropertyValue(`--${name}`).trim();
}

/** Increments whenever the effective theme changes, so canvas charts can repaint. */
export function subscribeTheme(onChange: () => void): () => void {
  const media = window.matchMedia("(prefers-color-scheme: dark)");
  const observer = new MutationObserver(onChange);
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme"] });
  media.addEventListener("change", onChange);
  return () => {
    observer.disconnect();
    media.removeEventListener("change", onChange);
  };
}
