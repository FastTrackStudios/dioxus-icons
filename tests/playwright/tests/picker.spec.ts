import { test, expect, type Page } from "@playwright/test";
import { fileURLToPath } from "node:url";
import path from "node:path";

const HERE = path.dirname(fileURLToPath(import.meta.url));
const DOC_INDEX = path.resolve(
  HERE,
  "../../../target/doc/dioxus_icons/index.html",
);
const DOC_URL = "file://" + DOC_INDEX;

async function waitForInteractivePicker(page: Page) {
  await page.locator("#dxi-picker:not([hidden])").waitFor();
  await page
    .locator("#dxi-picker-grid .dxi-picker-cell")
    .first()
    .waitFor();
}

test.describe("docs homepage picker", () => {
  test("interactive picker becomes visible and replaces the static fallback", async ({
    page,
  }) => {
    await page.goto(DOC_URL);
    await waitForInteractivePicker(page);

    await expect(page.locator("#dxi-picker")).toBeVisible();

    const fallbackDisplay = await page
      .locator("#dioxus-icons-static-picker")
      .evaluate((el) => getComputedStyle(el).display);
    expect(fallbackDisplay).toBe("none");
  });

  test("interactive grid lays out cells in multiple columns", async ({
    page,
  }) => {
    await page.goto(DOC_URL);
    await waitForInteractivePicker(page);

    const grid = page.locator("#dxi-picker-grid");
    const trackCount = await grid.evaluate(
      (el) =>
        getComputedStyle(el)
          .gridTemplateColumns.split(" ")
          .filter((token) => token.length > 0).length,
    );
    expect(trackCount).toBeGreaterThan(2);

    const cellCount = await grid.locator(".dxi-picker-cell").count();
    expect(cellCount).toBeGreaterThanOrEqual(trackCount);
  });

  test("first row cells share a single offsetTop (JS columns match CSS)", async ({
    page,
  }) => {
    await page.goto(DOC_URL);
    await waitForInteractivePicker(page);

    const grid = page.locator("#dxi-picker-grid");
    const trackCount = await grid.evaluate(
      (el) =>
        getComputedStyle(el)
          .gridTemplateColumns.split(" ")
          .filter((token) => token.length > 0).length,
    );

    const tops = await grid
      .locator(".dxi-picker-cell")
      .evaluateAll((cells) =>
        cells.slice(0, 32).map((el) => (el as HTMLElement).offsetTop),
      );

    const firstRowTop = tops[0];
    const cellsInFirstRow = tops.filter((t) => t === firstRowTop).length;
    expect(cellsInFirstRow).toBe(trackCount);

    const distinctTops = new Set(tops).size;
    expect(distinctTops).toBeGreaterThan(1);
  });

  test("typing in the search narrows the result count", async ({ page }) => {
    await page.goto(DOC_URL);
    await waitForInteractivePicker(page);

    const count = page.locator("#dxi-picker-count strong");
    const total = Number(await count.innerText());
    expect(total).toBeGreaterThan(100);

    await page.locator("#dxi-picker-input").fill("trash");
    await expect
      .poll(async () => Number(await count.innerText()))
      .toBeLessThan(total);

    const firstHref = await page
      .locator("#dxi-picker-grid .dxi-picker-cell")
      .first()
      .getAttribute("href");
    expect(firstHref).toMatch(/lucide\/fn\.[A-Za-z0-9_]*[Tt]rash[A-Za-z0-9_]*\.html/);
  });

  test("'/' keystroke focuses the search input", async ({ page }) => {
    await page.goto(DOC_URL);
    await waitForInteractivePicker(page);

    await page.locator("body").click({ position: { x: 1, y: 1 } });
    await page.keyboard.press("/");
    await expect(page.locator("#dxi-picker-input")).toBeFocused();
  });

  test("ArrowRight moves the active cell to the next sibling", async ({
    page,
  }) => {
    await page.goto(DOC_URL);
    await waitForInteractivePicker(page);

    const input = page.locator("#dxi-picker-input");
    await input.focus();
    await page.keyboard.press("ArrowRight");

    const activeIndex = await page
      .locator('#dxi-picker-grid [data-active="true"]')
      .getAttribute("data-index");
    expect(activeIndex).toBe("1");
  });

  test("each cell links to its lucide function page", async ({ page }) => {
    await page.goto(DOC_URL);
    await waitForInteractivePicker(page);

    const hrefs = await page
      .locator("#dxi-picker-grid .dxi-picker-cell")
      .evaluateAll((cells) =>
        cells.slice(0, 8).map((el) => (el as HTMLAnchorElement).getAttribute("href")),
      );
    expect(hrefs.length).toBeGreaterThan(0);
    for (const href of hrefs) {
      expect(href).toMatch(/^lucide\/fn\.[A-Za-z0-9_]+\.html$/);
    }
  });
});

test.describe("docs homepage picker (no JavaScript)", () => {
  test.use({ javaScriptEnabled: false });

  test("static fallback renders as a CSS grid", async ({ page }) => {
    await page.goto(DOC_URL);

    const fallback = page.locator("#dioxus-icons-static-picker");
    await expect(fallback).toBeVisible();

    const firstGrid = page.locator(".dioxus-icons-static-grid").first();
    const display = await firstGrid.evaluate(
      (el) => getComputedStyle(el).display,
    );
    expect(display).toBe("grid");

    const trackCount = await firstGrid.evaluate(
      (el) =>
        getComputedStyle(el)
          .gridTemplateColumns.split(" ")
          .filter((token) => token.length > 0).length,
    );
    expect(trackCount).toBeGreaterThan(2);

    const cellTops = await firstGrid
      .locator(".dioxus-icons-static-cell")
      .evaluateAll((cells) =>
        cells.slice(0, 16).map((el) => (el as HTMLElement).offsetTop),
      );
    const distinctTops = new Set(cellTops).size;
    expect(distinctTops).toBeLessThanOrEqual(2);

    const firstHref = await firstGrid
      .locator(".dioxus-icons-static-cell")
      .first()
      .getAttribute("href");
    expect(firstHref).toMatch(/^lucide\/fn\.[A-Za-z0-9_]+\.html$/);
  });
});
