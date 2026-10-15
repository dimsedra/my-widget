import { test, expect } from "@playwright/test";

test.describe("WidgetShell Visual & Transparency Baseline", () => {
  test("asserts transparent background and glassmorphism styling baseline", async ({
    page,
  }) => {
    await page.goto("/");

    // 1. Verify container and drag handle render
    const shell = page.locator('[data-testid="widget-shell-container"]');
    await expect(shell).toBeVisible();

    const dragHandle = page.locator('[data-testid="widget-drag-handle"]');
    await expect(dragHandle).toBeVisible();
    await expect(dragHandle).toHaveAttribute("data-tauri-drag-region");

    // 2. Body transparency check (must be transparent rgba(0, 0, 0, 0) for Tauri desktop overlay)
    const bodyBg = await page.evaluate(() => {
      return window.getComputedStyle(document.body).backgroundColor;
    });
    expect(bodyBg).toBe("rgba(0, 0, 0, 0)");

    // 3. Root html transparency check
    const htmlBg = await page.evaluate(() => {
      return window.getComputedStyle(document.documentElement).backgroundColor;
    });
    expect(htmlBg).toBe("rgba(0, 0, 0, 0)");

    // 4. Shell container visual snapshot baseline
    await expect(shell).toHaveScreenshot("widget-shell-baseline.png", {
      maxDiffPixelRatio: 0.05,
    });
  });
});
