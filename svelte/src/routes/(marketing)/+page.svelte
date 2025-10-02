<script lang="ts">
  import {
    IntroCard,
    LargeTitle,
    Merger,
    Toolbar,
    UniButton,
    HStack,
    Divider,
    WorkArea,
    MetaData,
  } from "$lib/components";
  import { Flame, FlameKindling } from "@lucide/svelte";
  import { m } from "$lib/paraglide/messages";
  const seoData = {
    title: "Ogonek - Digital Classroom for Private Teachers & Tutors",
    description:
      "Transform your teaching with Ogonek's digital classroom. Create interactive flashcards, manage lessons, track student progress, and organize your private tutoring business efficiently.",
    keywords:
      "digital classroom, private tutoring, teaching platform, flashcards, lesson management, student tracking, education technology, online teaching tools, tutor software",
    ogTitle: "Ogonek - Digital Classroom for Teachers",
    ogType: "website" as const,
    jsonLd: {
      "@context": "https://schema.org",
      "@type": "WebApplication",
      name: "Ogonek",
      url: "https://ogonek.app",
      description:
        "Transform your teaching with Ogonek's digital classroom. Create interactive flashcards, manage lessons, track student progress, and organize your private tutoring business efficiently.",
      applicationCategory: "EducationalApplication",
      operatingSystem: "All",
      featureList: [
        "Interactive Flashcards Creation",
        "Lesson Planning and Scheduling",
        "Student Progress Tracking",
        "Digital Lesson Notes",
        "Task and Assignment Management",
        "Private Tutoring Business Organization",
      ],
      author: {
        "@type": "Organization",
        name: "noxlovette",
        url: "https://noxlovette.com",
      },
      offers: {
        "@type": "Offer",
        price: "0",
        priceCurrency: "USD",
        category: "Free",
      },
      screenshot: "https://ogonek.app/images/og.png",
      potentialAction: [
        {
          "@type": "RegisterAction",
          target: "https://ogonek.app/signup",
          name: "Sign Up for Ogonek",
        },
        {
          "@type": "LoginAction",
          target: "https://ogonek.app/login",
          name: "Login to Ogonek",
        },
      ],
    },
  };
</script>

<WorkArea>
  <Toolbar>
    <HStack>
      <LargeTitle>
        {m.welcomeTo()}<span class="font-serif italic">Ogonëk</span>
      </LargeTitle>
      <Divider />
      <Merger>
        <UniButton
          content={m.logIn()}
          iconOnly={false}
          Icon={Flame}
          dataCy="login-button"
          href="/login"
        ></UniButton>
        <UniButton
          iconOnly={false}
          Icon={FlameKindling}
          href="/signup"
          dataCy="signup-button"
          variant="primary"
          content={m.signUp()}
        ></UniButton>
      </Merger>
    </HStack>
  </Toolbar>
  <HStack>
    <IntroCard title={m.lessons()}>{m.mellow_ok_blackbird_peek()}</IntroCard>
    <IntroCard title={m.livid_small_gadfly_gulp()}
      >{m.small_deft_goose_grin()}</IntroCard
    >
    <IntroCard title={m.flashcards()}>{m.calm_blue_warbler_zoom()}</IntroCard>
  </HStack>
</WorkArea>

<MetaData {...seoData} />
