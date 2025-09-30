import type { RequestHandler } from "./$types";

interface SitemapUrl {
  loc: string;
  lastmod?: string;
  changefreq?:
    | "always"
    | "hourly"
    | "daily"
    | "weekly"
    | "monthly"
    | "yearly"
    | "never";
  priority?: number;
}

const SITE_URL = "https://ogonek.app";

// Static routes that should be included in sitemap
const staticRoutes: SitemapUrl[] = [
  {
    loc: "",
    lastmod: new Date().toISOString().split("T")[0],
    changefreq: "monthly",
    priority: 1.0,
  },
  {
    loc: "login",
    lastmod: new Date().toISOString().split("T")[0],
    changefreq: "monthly",
    priority: 0.8,
  },
  {
    loc: "signup",
    lastmod: new Date().toISOString().split("T")[0],
    changefreq: "monthly",
    priority: 0.8,
  },
  {
    loc: "contact",
    lastmod: new Date().toISOString().split("T")[0],
    changefreq: "monthly",
    priority: 0.7,
  },
  {
    loc: "privacy",
    lastmod: new Date().toISOString().split("T")[0],
    changefreq: "yearly",
    priority: 0.5,
  },
  {
    loc: "terms",
    lastmod: new Date().toISOString().split("T")[0],
    changefreq: "yearly",
    priority: 0.5,
  },
];

// Languages your app supports (based on your [[lang=lang]] structure)
const languages = ["ru", "en"]; // Empty string for default, 'en' for explicit English

function generateSitemap(urls: SitemapUrl[]): string {
  const urlElements = urls
    .map((url) => {
      let urlElement = `  <url>\n    <loc>${url.loc}</loc>`;

      if (url.lastmod) {
        urlElement += `\n    <lastmod>${url.lastmod}</lastmod>`;
      }

      if (url.changefreq) {
        urlElement += `\n    <changefreq>${url.changefreq}</changefreq>`;
      }

      if (url.priority !== undefined) {
        urlElement += `\n    <priority>${url.priority}</priority>`;
      }

      urlElement += "\n  </url>";
      return urlElement;
    })
    .join("\n");

  return `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
${urlElements}
</urlset>`;
}

export const GET: RequestHandler = async () => {
  try {
    const allUrls: SitemapUrl[] = [];

    // Generate URLs for each language
    for (const lang of languages) {
      for (const route of staticRoutes) {
        const langPrefix = lang ? `/${lang}` : "";
        const routePath = route.loc ? `/${route.loc}` : "";

        allUrls.push({
          ...route,
          loc: `${SITE_URL}${langPrefix}${routePath}`,
        });
      }
    }

    const sitemap = generateSitemap(allUrls);

    return new Response(sitemap, {
      status: 200,
      headers: {
        "Content-Type": "application/xml",
        "Cache-Control": "public, max-age=3600", // Cache for 1 hour
      },
    });
  } catch (error) {
    console.error("Error generating sitemap:", error);
    return new Response("Error generating sitemap", { status: 500 });
  }
};
