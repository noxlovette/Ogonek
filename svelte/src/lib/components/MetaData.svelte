<script lang="ts">
  import { page } from "$app/state";

  interface Props {
    description?: string;
    title?: string;
    keywords?: string;
    ogTitle?: string;
    ogUrl?: string;
    ogImage?: string;
    ogType?: string;
    twitterCard?: string;
    twitterSite?: string;
    canonicalUrl?: string;
    jsonLd?: object;
  }

  let {
    description = "Ogonek. The digital classroom for private teachers.",
    title = "Ogonek",
    keywords = "education, teach, English, classroom management, teaching, learning, private tutoring, flashcards, lesson planning",
    ogTitle = "Ogonek",
    ogUrl = "https://ogonek.app",
    ogImage = "https://ogonek.app/images/og.png",
    ogType = "website",
    twitterCard = "summary_large_image",
    twitterSite = "@ogonek_app",
    canonicalUrl,
    jsonLd,
  }: Props = $props();

  const currentCanonical = $derived(
    canonicalUrl || `https://ogonek.app${page.url.pathname}`,
  );

  const defaultJsonLd = {
    "@context": "https://schema.org",
    "@type": "WebApplication",
    name: "Ogonek",
    url: "https://ogonek.app",
    description: description,
    applicationCategory: "EducationalApplication",
    operatingSystem: "All",
    featureList: [
      "Interactive Flashcards",
      "Lesson Scheduling",
      "Student Progress Tracking",
      "Lesson Notes",
      "Task Management",
    ],
    softwareVersion: "1.9.0",
    author: {
      "@type": "Organization",
      name: "noxlovette",
      url: "https://noxlovette.com",
    },
    datePublished: "2024-12-17",
    inLanguage: "en-GB",
    screenshot: "https://ogonek.app/images/og.png",
    offers: {
      "@type": "Offer",
      price: "0",
      priceCurrency: "USD",
      category: "Free",
    },
  };

  const finalJsonLd = $derived(jsonLd || defaultJsonLd);
</script>

<svelte:head>
  <title>{title}</title>
  <meta name="apple-mobile-web-app-capable" content="yes" />
  <meta
    name="apple-mobile-web-app-status-bar-style"
    content="black-translucent"
  />
  <meta
    name="viewport"
    content="width=device-width, initial-scale=1, viewport-fit=cover"
  />
  <meta
    name="theme-color"
    content="#fafaf9"
    media="(prefers-color-scheme: light)"
  />
  <meta
    name="theme-color"
    content="#0c0a09"
    media="(prefers-color-scheme: dark)"
  />
  <meta name="description" content={description} />
  <meta name="keywords" content={keywords} />
  <meta property="og:locale" content="en_GB" />

  <!-- Canonical URL -->
  <link rel="canonical" href={currentCanonical} />

  <!-- Open Graph -->
  <meta property="og:title" content={ogTitle} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={currentCanonical} />
  <meta property="og:image" content={ogImage} />
  <meta property="og:image:alt" content={`${ogTitle} - ${description}`} />
  <meta property="og:image:width" content="1200" />
  <meta property="og:image:height" content="630" />
  <meta property="og:type" content={ogType} />
  <meta property="og:site_name" content="Ogonek" />

  <meta name="twitter:card" content={twitterCard} />
  <meta name="twitter:site" content={twitterSite} />
  <meta name="twitter:creator" content="@noxlovette" />
  <meta name="twitter:title" content={ogTitle} />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:image" content={ogImage} />
  <meta name="twitter:image:alt" content={`${ogTitle} - ${description}`} />

  <meta name="author" content="noxlovette" />
  <meta name="language" content="en-GB" />
  <meta name="rating" content="general" />
  <meta name="revisit-after" content="7 days" />

  <script type="application/ld+json">
    {JSON.stringify(finalJsonLd)}
  </script>
</svelte:head>
