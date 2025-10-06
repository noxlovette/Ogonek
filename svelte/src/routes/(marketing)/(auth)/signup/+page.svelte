<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Captcha,
    UniButton,
    Input,
    Grid,
    Merger,
    MetaData,
  } from "$lib/components";
  import { DoorOpen } from "@lucide/svelte";
  import { enhanceForm } from "$lib/utils";
  import type { PageProps } from "./$types";
  import message from "$lib/texts";
  import { goto } from "$app/navigation";

  let password = $state("");
  let confirmPassword = $state("");
  let passwordMatch = $state(true);

  let { form }: PageProps = $props();

  const seoData = {
    title: "Sign Up for Ogonek - Create Your Teaching Account",
    description:
      "Join thousands of teachers using Ogonek's digital classroom. Create flashcards, manage lessons, track student progress. Free account for private tutors and teachers.",
    keywords:
      "sign up, create account, teacher registration, join ogonek, digital classroom registration, teaching platform signup",
    ogTitle: "Sign Up for Ogonek - Digital Teaching Platform",
    ogType: "website" as const,
    jsonLd: {
      "@context": "https://schema.org",
      "@type": "WebPage",
      name: "Sign Up for Ogonek",
      description:
        "Join thousands of teachers using Ogonek's digital classroom. Create your free teaching account today.",
      url: "https://ogonek.app/signup",
      isPartOf: {
        "@type": "WebSite",
        name: "Ogonek",
        url: "https://ogonek.app",
      },
      potentialAction: {
        "@type": "RegisterAction",
        target: "https://ogonek.app/signup",
        name: "Create Ogonek Account",
      },
    },
  };
</script>

<form
  method="post"
  class="gap-default flex max-w-md flex-col"
  use:enhance={enhanceForm({
    messages: {
      redirect: message.auth.signupSuccess,
      defaultError: message.auth.signupFailure,
    },
    handlers: {
      success: async () => {
        await goto("/login");
      },
    },
    navigate: true,
    shouldUpdate: true,
  })}
>
  <Grid>
    <Input
      name="name"
      showLabel={false}
      placeholder="Как вас называть?"
      value=""
      dataCy="name-field"
      invalid={form?.name}
      invalidDescription="3+ символов"
    ></Input>
    <Input
      name="username"
      showLabel={false}
      placeholder="Ваш ник"
      dataCy="username-field"
      value=""
      invalid={form?.username}
      invalidDescription="2+ символов"
    ></Input>

    <Input name="role" dataCy="role-field" showLabel={false} type="role" />

    <Input
      name="email"
      dataCy="email-field"
      invalid={form?.email}
      invalidDescription="Это не почта"
      showLabel={false}
      placeholder="Почта"
      type="email"
      value=""
    ></Input>

    <Input
      name="pass"
      invalid={form?.pass}
      invalidDescription="8+ символов"
      placeholder="Пароль"
      showLabel={false}
      type="password"
      bind:value={password}
    ></Input>
    <Input
      name="confirmPassword"
      placeholder="И еще раз"
      invalid={form?.confirmPassword}
      invalidDescription="Пароли не совпадают"
      showLabel={false}
      type="password"
      bind:value={confirmPassword}
    ></Input>
  </Grid>
  <Captcha />
  <Merger>
    <UniButton
      content="Регистрация"
      Icon={DoorOpen}
      type="submit"
      variant="primary"
      iconOnly={false}
    ></UniButton>
  </Merger>
</form>
<MetaData {...seoData} />
