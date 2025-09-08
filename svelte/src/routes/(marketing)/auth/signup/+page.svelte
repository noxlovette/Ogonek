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
  import { DoorOpen } from "lucide-svelte";
  import { enhanceForm } from "$lib/utils";
  import { m } from "$lib/paraglide/messages";
  import type { PageProps } from "./$types";
  import message from "$lib/messages";

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
      url: "https://ogonek.app/auth/signup",
      isPartOf: {
        "@type": "WebSite",
        name: "Ogonek",
        url: "https://ogonek.app",
      },
      potentialAction: {
        "@type": "RegisterAction",
        target: "https://ogonek.app/auth/signup",
        name: "Create Ogonek Account",
      },
    },
  };
</script>

<form
  method="post"
  class="flex max-w-md flex-col gap-2 md:gap-3 lg:gap-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: message.auth.signupSuccess,
      defaultError: message.auth.signupFailure,
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
      invalid={form?.name}
      invalidDescription="3+ characters"
    ></Input>
    <Input
      name="username"
      showLabel={false}
      placeholder="Username"
      value=""
      invalid={form?.username}
      invalidDescription="2+ characters"
    ></Input>

    <Input name="role" showLabel={false} type="role" />

    <Input
      name="email"
      invalid={form?.email}
      invalidDescription="Invalid email"
      showLabel={false}
      placeholder="Email"
      type="email"
      value=""
    ></Input>

    <Input
      name="password"
      invalid={form?.pass}
      invalidDescription="3+ characters"
      placeholder="Password"
      showLabel={false}
      type="password"
      bind:value={password}
    ></Input>
    <Input
      name="confirmPassword"
      placeholder="Again"
      showLabel={false}
      type="password"
      bind:value={confirmPassword}
    ></Input>

    {#if !passwordMatch}
      <p class="mt-1 text-sm text-red-600">Passwords don't match</p>
    {/if}
  </Grid>
  <Captcha />
  <Merger>
    <UniButton Icon={DoorOpen} type="submit" variant="primary" iconOnly={false}
      >{m.signUp()}</UniButton
    >
  </Merger>
</form>
<MetaData {...seoData} />
