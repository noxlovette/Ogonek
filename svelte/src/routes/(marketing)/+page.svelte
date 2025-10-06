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
    HLine,
  } from "$lib/components";
  import Title1 from "$lib/components/typography/Title1.svelte";
  import Grid from "$lib/components/UI/layout/2Grid.svelte";
  import { Flame, FlameKindling } from "@lucide/svelte";
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

<HStack styling="items-center text-center">
  <LargeTitle>Преподавание — ваше призвание</LargeTitle>
  <Title1>Организация — наше</Title1>

  <HLine></HLine>

  <Grid>
    <IntroCard title="Карточки">
      Создавайте и находите колоды с нужными словами и фразами, назначайте их
      ученикам и отслеживайте прогресс. Больше никаких скучных списков новой
      лексики.</IntroCard
    >
    <IntroCard title="Занятия">
      Структурируйте материалы, отслеживайте прогресс и давайте обратную связь
      по каждому занятию. Конспекты, презентации, успехи – все в одном месте.
    </IntroCard>

    <IntroCard title="Задания">
      Назначайте, храните и проверяйте домашнюю работу для всех учеников в
      едином организованном пространстве.
    </IntroCard>

    <IntroCard title="Календарь"
      >Ваше расписание - под полным контролем. Назначайте занятия,
      согласовывайте переносы в пару кликов и автоматически уведомляйте
      учеников. Забудьте о пробелах в расписании и пропущенных занятиях.</IntroCard
    >
  </Grid>
  <HLine></HLine>
  <Merger>
    <UniButton
      content="Авторизация"
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
      content="Регистрация"
    ></UniButton>
  </Merger>
</HStack>

<MetaData {...seoData} />
<svelte:head>
  <meta name="robots" content="index, follow" />
</svelte:head>
