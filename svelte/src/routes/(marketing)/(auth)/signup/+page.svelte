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
  import { m } from "$lib/paraglide/messages";
  import type { PageProps } from "./$types";
  import message from "$lib/messages";
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
      placeholder="Name"
      value=""
      dataCy="name-field"
      invalid={form?.name}
      invalidDescription={`3+ ${m.factual_caring_blackbird_ripple()}`}
    ></Input>
    <Input
      name="username"
      showLabel={false}
      placeholder="Username"
      dataCy="username-field"
      value=""
      invalid={form?.username}
      invalidDescription={`2+ ${m.factual_caring_blackbird_ripple()}`}
    ></Input>

    <Input name="role" dataCy="role-field" showLabel={false} type="role" />

    <Input
      name="email"
      dataCy="email-field"
      invalid={form?.email}
      invalidDescription={m.direct_big_ape_yell()}
      showLabel={false}
      placeholder="Email"
      type="email"
      value=""
    ></Input>

    <Input
      name="pass"
      invalid={form?.pass}
      invalidDescription={`8+ ${m.factual_caring_blackbird_ripple()}`}
      placeholder="Password"
      showLabel={false}
      type="password"
      bind:value={password}
    ></Input>
    <Input
      name="confirmPassword"
      placeholder="Again"
      invalid={form?.confirmPassword}
      invalidDescription="Пароли не совпадают"
      showLabel={false}
      type="password"
      bind:value={confirmPassword}
    ></Input>

    {#if !passwordMatch}
      <p class="mt-1 text-sm text-red-600">
        {m.extra_grand_angelfish_transform()}
      </p>
    {/if}
  </Grid>
  <Captcha />
  <Merger>
    <UniButton
      content={m.signUp()}
      Icon={DoorOpen}
      type="submit"
      variant="primary"
      iconOnly={false}
    ></UniButton>
  </Merger>
</form>
<MetaData {...seoData} />
