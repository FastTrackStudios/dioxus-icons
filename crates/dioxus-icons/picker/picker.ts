interface IconEntry {
  name: string;
  svg: string;
  tags?: string[];
  categories?: string[];
}

interface IconManifest {
  icons: IconEntry[];
}

type Rank = 0 | 1 | 2 | 9;

const DEFAULT_ROW_HEIGHT = 92;
const DEFAULT_CELL_MIN_WIDTH = 96;
const ROW_GAP = 4;

(function init(): void {
  const manifestNode = document.getElementById("__icon_manifest__");
  if (!manifestNode) return;

  let manifest: IconManifest;
  try {
    manifest = JSON.parse(manifestNode.textContent || '{"icons":[]}') as IconManifest;
  } catch {
    return;
  }
  const icons: IconEntry[] = Array.isArray(manifest.icons) ? manifest.icons : [];

  const roots = Array.from(
    document.querySelectorAll<HTMLElement>("[data-dxi-picker]"),
  );
  if (!roots.length) return;

  const fallback = document.getElementById("dioxus-icons-static-picker");
  if (fallback) (fallback as HTMLElement).style.display = "none";

  roots.forEach((root) => {
    root.hidden = false;
    bindPicker(root, icons);
  });

  document.addEventListener("keydown", (event: KeyboardEvent) => {
    if (event.key !== "/" || isTyping(event.target)) return;
    const visible = findVisibleInput(roots);
    if (!visible) return;
    event.preventDefault();
    visible.focus();
    visible.select();
  });
})();

function findVisibleInput(roots: HTMLElement[]): HTMLInputElement | null {
  for (const root of roots) {
    if (root.offsetParent === null && root.getClientRects().length === 0) continue;
    const input = root.querySelector<HTMLInputElement>("[data-dxi-input]");
    if (input && input.offsetParent !== null) return input;
  }
  return null;
}

function isTyping(target: EventTarget | null): boolean {
  if (!target || !(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  return tag === "INPUT" || tag === "TEXTAREA" || target.isContentEditable;
}

function bindPicker(root: HTMLElement, icons: IconEntry[]): void {
  const input = root.querySelector<HTMLInputElement>("[data-dxi-input]");
  const clearBtn = root.querySelector<HTMLButtonElement>("[data-dxi-clear]");
  const kbd = root.querySelector<HTMLElement>("[data-dxi-kbd]");
  const count = root.querySelector<HTMLElement>("[data-dxi-count]");
  const scroller = root.querySelector<HTMLElement>("[data-dxi-scroll]");
  const grid = root.querySelector<HTMLElement>("[data-dxi-grid]");
  const topSpacer = root.querySelector<HTMLElement>("[data-dxi-top]");
  const bottomSpacer = root.querySelector<HTMLElement>("[data-dxi-bottom]");

  if (!input || !scroller || !grid || !topSpacer || !bottomSpacer) return;

  let filtered: IconEntry[] = icons.slice();
  let activeIndex = 0;
  let columns = 1;
  let rowHeight = DEFAULT_ROW_HEIGHT;

  function lower(value: unknown): string {
    return String(value ?? "").toLowerCase();
  }

  function rank(icon: IconEntry, query: string): Rank {
    if (!query) return 0;
    if (lower(icon.name).indexOf(query) !== -1) return 0;
    const tags = icon.tags ?? [];
    for (const tag of tags) {
      if (lower(tag).indexOf(query) !== -1) return 1;
    }
    const categories = icon.categories ?? [];
    for (const category of categories) {
      if (lower(category).indexOf(query) !== -1) return 2;
    }
    return 9;
  }

  function filter(): void {
    const query = lower(input!.value).trim();
    filtered = icons
      .map((icon) => ({ icon, rank: rank(icon, query) }))
      .filter((item) => item.rank !== 9)
      .sort((a, b) =>
        a.rank !== b.rank ? a.rank - b.rank : a.icon.name.localeCompare(b.icon.name),
      )
      .map((item) => item.icon);
    activeIndex = 0;
    scroller!.scrollTop = 0;
    if (clearBtn) clearBtn.hidden = !query.length;
    if (kbd) kbd.hidden = query.length > 0;
    render();
  }

  function measure(): void {
    const computed = window.getComputedStyle(grid!);
    const template = computed.gridTemplateColumns;
    const trackCount =
      template && template !== "none"
        ? template.split(" ").filter((token) => token.length > 0).length
        : 0;
    if (trackCount > 0) {
      columns = trackCount;
    } else {
      const width = grid!.clientWidth || scroller!.clientWidth || 640;
      const padX =
        (parseFloat(computed.paddingLeft) || 0) +
        (parseFloat(computed.paddingRight) || 0);
      const gap = parseFloat(computed.columnGap) || ROW_GAP;
      const available = Math.max(0, width - padX) + gap;
      columns = Math.max(1, Math.floor(available / (DEFAULT_CELL_MIN_WIDTH + gap)));
    }

    const firstCell = grid!.firstElementChild as HTMLElement | null;
    if (firstCell && firstCell.offsetHeight > 0) {
      const gap = parseFloat(computed.rowGap) || ROW_GAP;
      rowHeight = firstCell.offsetHeight + gap;
    }
  }

  function hrefFor(icon: IconEntry): string {
    return "lucide/fn." + icon.name + ".html";
  }

  function appendCell(icon: IconEntry, index: number): void {
    const link = document.createElement("a");
    link.className = "dxi-picker-cell";
    link.href = hrefFor(icon);
    link.dataset.index = String(index);
    link.dataset.active = index === activeIndex ? "true" : "false";
    link.tabIndex = index === activeIndex ? 0 : -1;
    const tagSummary = (icon.tags ?? []).slice(0, 4).join(", ");
    link.title = tagSummary ? icon.name + " — " + tagSummary : icon.name;

    const preview = document.createElement("span");
    preview.className = "dxi-picker-cell-preview";
    preview.innerHTML = icon.svg || "";

    const name = document.createElement("span");
    name.className = "dxi-picker-cell-name";
    name.textContent = icon.name;

    link.appendChild(preview);
    link.appendChild(name);
    grid!.appendChild(link);
  }

  function renderEmpty(): void {
    const empty = document.createElement("div");
    empty.className = "dxi-picker-empty";

    const iconSvg =
      '<svg class="dxi-picker-empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true"><circle cx="11" cy="11" r="8"></circle><path d="m21 21-4.3-4.3"></path><path d="M8.5 11h5"></path></svg>';
    empty.innerHTML = iconSvg;

    const title = document.createElement("div");
    title.className = "dxi-picker-empty-title";
    title.textContent = "No icons match";

    const hint = document.createElement("div");
    hint.className = "dxi-picker-empty-hint";
    hint.textContent = "Try a shorter or different search term.";

    empty.appendChild(title);
    empty.appendChild(hint);
    grid!.appendChild(empty);
  }

  function render(): void {
    measure();
    const total = filtered.length;
    if (count) {
      count.innerHTML =
        total === icons.length
          ? "<strong>" + total + "</strong> icons"
          : "<strong>" + total + "</strong> of " + icons.length;
    }
    grid!.textContent = "";

    if (!total) {
      topSpacer!.style.height = "0px";
      bottomSpacer!.style.height = "0px";
      renderEmpty();
      return;
    }

    const totalRows = Math.ceil(total / columns);
    const visibleRows = Math.ceil(scroller!.clientHeight / rowHeight) + 4;
    const startRow = Math.max(0, Math.floor(scroller!.scrollTop / rowHeight) - 2);
    const endRow = Math.min(totalRows, startRow + visibleRows);
    const start = startRow * columns;
    const end = Math.min(total, endRow * columns);

    topSpacer!.style.height = startRow * rowHeight + "px";
    bottomSpacer!.style.height = Math.max(0, (totalRows - endRow) * rowHeight) + "px";

    for (let i = start; i !== end; i += 1) {
      appendCell(filtered[i], i);
    }
  }

  function ensureActiveVisible(): void {
    const row = Math.floor(activeIndex / columns);
    const top = row * rowHeight;
    const bottom = top + rowHeight;
    if (scroller!.scrollTop > top) scroller!.scrollTop = top;
    if (bottom > scroller!.scrollTop + scroller!.clientHeight) {
      scroller!.scrollTop = bottom - scroller!.clientHeight;
    }
    render();
    const active = grid!.querySelector<HTMLElement>('[data-active="true"]');
    if (active) active.focus({ preventScroll: true });
  }

  function handleNavigation(event: KeyboardEvent): void {
    const target = event.target;
    if (
      target !== input &&
      target !== scroller &&
      (!(target instanceof Node) || !grid!.contains(target))
    ) {
      return;
    }
    if (event.altKey || event.ctrlKey || event.metaKey) return;

    let nextIndex = activeIndex;
    if (event.key === "ArrowDown") {
      nextIndex += columns;
    } else if (event.key === "ArrowRight") {
      nextIndex += 1;
    } else if (event.key === "ArrowUp") {
      nextIndex -= columns;
    } else if (event.key === "ArrowLeft") {
      nextIndex -= 1;
    } else if (event.key === "Enter") {
      const activeCell =
        target instanceof HTMLElement
          ? target.closest<HTMLAnchorElement>(".dxi-picker-cell")
          : null;
      const cellIndex = activeCell ? Number(activeCell.dataset.index) : NaN;
      const icon =
        Number.isInteger(cellIndex) && filtered[cellIndex]
          ? filtered[cellIndex]
          : filtered[activeIndex];

      if (icon) {
        event.preventDefault();
        window.location.href = hrefFor(icon);
      }
      return;
    } else if (event.key === "Escape") {
      event.preventDefault();
      if (input!.value) {
        input!.value = "";
        filter();
      } else if (target === input) {
        input!.blur();
      } else {
        input!.focus();
        input!.select();
      }
      return;
    } else {
      return;
    }

    event.preventDefault();
    if (!filtered.length) return;
    activeIndex = Math.max(0, Math.min(filtered.length - 1, nextIndex));
    ensureActiveVisible();
  }

  input.addEventListener("input", filter);
  root.addEventListener("keydown", handleNavigation);

  if (clearBtn) {
    clearBtn.addEventListener("click", () => {
      input.value = "";
      filter();
      input.focus();
    });
  }

  scroller.addEventListener("scroll", () => {
    window.requestAnimationFrame(render);
  });

  window.addEventListener("resize", () => {
    window.requestAnimationFrame(render);
  });

  if ("IntersectionObserver" in window) {
    const observer = new IntersectionObserver(
      () => {
        window.requestAnimationFrame(render);
      },
      { root: scroller },
    );
    observer.observe(topSpacer);
    observer.observe(bottomSpacer);
  }

  if ("ResizeObserver" in window) {
    const ro = new ResizeObserver(() => {
      window.requestAnimationFrame(render);
    });
    ro.observe(scroller);
  }

  filter();
}
