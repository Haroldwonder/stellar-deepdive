export type ExportFormat = "png" | "svg";

interface ExportOptions {
  filename: string;
  format: ExportFormat;
  /** Device-pixel scale for PNG raster export. Defaults to 2 for crisp output. */
  scale?: number;
}

/** Fallback background used when the chart container has a transparent background. */
const FALLBACK_BACKGROUND = "#0f172a";

function getBackgroundColor(element: HTMLElement): string {
  const bg = getComputedStyle(element).backgroundColor;
  if (!bg || bg === "transparent" || bg === "rgba(0, 0, 0, 0)") {
    return FALLBACK_BACKGROUND;
  }
  return bg;
}

/** Clone the chart's <svg> with explicit dimensions and namespace so it stands alone. */
function serializeSvg(svg: SVGSVGElement): { source: string; width: number; height: number } {
  const rect = svg.getBoundingClientRect();
  const width = svg.clientWidth || rect.width;
  const height = svg.clientHeight || rect.height;

  const clone = svg.cloneNode(true) as SVGSVGElement;
  clone.setAttribute("xmlns", "http://www.w3.org/2000/svg");
  clone.setAttribute("width", String(width));
  clone.setAttribute("height", String(height));
  if (!clone.getAttribute("viewBox")) {
    clone.setAttribute("viewBox", `0 0 ${width} ${height}`);
  }

  const source = new XMLSerializer().serializeToString(clone);
  return { source, width, height };
}

function triggerDownload(blob: Blob, filename: string) {
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = filename;
  link.style.visibility = "hidden";
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(url);
}

/**
 * Export a chart container element to an image file. Locates the chart's SVG
 * (e.g. rendered by recharts) and either downloads it directly as SVG or
 * rasterizes it onto a canvas for PNG.
 */
export async function exportChart(
  element: HTMLElement,
  { filename, format, scale = 2 }: ExportOptions,
): Promise<void> {
  const svg = element.querySelector("svg");
  if (!svg) {
    throw new Error("No chart SVG found to export");
  }

  const { source, width, height } = serializeSvg(svg as SVGSVGElement);

  if (format === "svg") {
    const blob = new Blob([source], { type: "image/svg+xml;charset=utf-8" });
    triggerDownload(blob, `${filename}.svg`);
    return;
  }

  // PNG: draw the serialized SVG onto a canvas.
  const svgBlob = new Blob([source], { type: "image/svg+xml;charset=utf-8" });
  const svgUrl = URL.createObjectURL(svgBlob);

  try {
    const image = await new Promise<HTMLImageElement>((resolve, reject) => {
      const img = new window.Image();
      img.onload = () => resolve(img);
      img.onerror = () => reject(new Error("Failed to render chart image"));
      img.src = svgUrl;
    });

    const canvas = document.createElement("canvas");
    canvas.width = width * scale;
    canvas.height = height * scale;
    const ctx = canvas.getContext("2d");
    if (!ctx) {
      throw new Error("Canvas 2D context unavailable");
    }

    ctx.scale(scale, scale);
    ctx.fillStyle = getBackgroundColor(element);
    ctx.fillRect(0, 0, width, height);
    ctx.drawImage(image, 0, 0, width, height);

    const pngBlob = await new Promise<Blob | null>((resolve) =>
      canvas.toBlob(resolve, "image/png"),
    );
    if (!pngBlob) {
      throw new Error("Failed to encode PNG");
    }
    triggerDownload(pngBlob, `${filename}.png`);
  } finally {
    URL.revokeObjectURL(svgUrl);
  }
}
