<script lang="ts">
  interface Props {
    description?: string;
    title?: string;
    keywords?: string;
    robots?: string;
    ogTitle?: string;
    ogUrl?: string;
    ogImage?: string;
  }

  let {
    description = "Ogonek. The digital classroom for private teachers.",
    title = "Ogonek",
    keywords = "education, teach, English, classroom management, teaching, learning",
    robots = "index, follow",
    ogTitle = "Ogonek",
    ogUrl = "https://ogonek.app",
    ogImage = "https://ogonek.app/images/og.png",
  }: Props = $props();
  let ogType = "website";
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
  <meta name="robots" content={robots} />
  <meta property="og:locale" content="en_GB" />

  <meta property="og:title" content={ogTitle} />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={ogUrl} />
  <meta property="og:image" content={ogImage} />
  <meta property="og:type" content={ogType} />

  <script type="application/ld+json">
    {
      "@context": "http://schema.org",
      "@type": "WebApplication",
      "name": "Ogonek",
      "url": "https://ogonek.app",
      "description": "Ogonek. The digital classroom for private teachers.",
      "applicationCategory": "EducationalApplication",
      "operatingSystem": "All",
      // "screenshot": "https://Ogonek.noxlovette.com/static/images/screenshot.jpg",
      "featureList": "Flashcards, Scheduling, Lesson Notes, Student Progress Tracking",
      "softwareVersion": "1.2.11",
      "author": {
        "@type": "Organization",
        "name": "noxlovette",
        "url": "https://noxlovette.com"
      },
      "datePublished": "2024-12-17"
    }
  </script>
</svelte:head>
