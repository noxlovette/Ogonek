<script lang="ts">
  import { enhance } from "$app/forms";
  import {
    Input,
    Captcha,
    UniButton,
    Grid,
    Merger,
    MetaData,
  } from "$lib/components";
  import message from "$lib/messages.js";
  import { m } from "$lib/paraglide/messages";
  import { enhanceForm } from "$lib/utils";
  import { DoorOpen } from "lucide-svelte";

  let { form } = $props();

  const seoData = {
    title: "Login to Ogonek - Digital Classroom for Teachers",
    description:
      "Sign in to your Ogonek account and access your digital teaching tools. Manage lessons, track student progress, and organize your private tutoring business.",
    keywords:
      "login, sign in, teacher login, digital classroom, education platform, teaching tools, tutor login",
    ogTitle: "Login to Ogonek",
    ogType: "website" as const,
    jsonLd: {
      "@context": "https://schema.org",
      "@type": "WebPage",
      name: "Login to Ogonek",
      description:
        "Sign in to your Ogonek account and access your digital teaching tools.",
      url: "https://ogonek.app/login",
      isPartOf: {
        "@type": "WebSite",
        name: "Ogonek",
        url: "https://ogonek.app",
      },
      potentialAction: {
        "@type": "LoginAction",
        target: "https://ogonek.app/login",
        name: "Login to Ogonek",
      },
    },
  };
</script>

<form
  method="POST"
  class="flex max-w-md flex-col gap-2 md:gap-3 lg:gap-4"
  use:enhance={enhanceForm({
    messages: {
      redirect: message.auth.loginSuccess,
    },
  })}
>
  <Grid>
    <Input
      required={true}
      showLabel={false}
      name="username"
      invalid={form?.username}
      placeholder="Username"
      invalidDescription="2+ characters"
      value=""
    />

    <Input
      required={true}
      name="pass"
      showLabel={false}
      invalid={form?.pass}
      placeholder="Password"
      invalidDescription="3+ characters"
      value=""
      type="password"
    />
  </Grid>
  <Captcha />
  <Merger>
    <UniButton
      Icon={DoorOpen}
      type="submit"
      variant="primary"
      ariaLabel="Login"
      iconOnly={false}>{m.logIn()}</UniButton
    >
  </Merger>
</form>

<MetaData {...seoData} />
