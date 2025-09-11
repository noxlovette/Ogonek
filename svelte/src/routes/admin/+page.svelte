<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
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
  import { initialUser, notification } from "$lib/stores";
  import { enhanceForm } from "$lib/utils";
  import { DoorOpen } from "lucide-svelte";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

ADMIN PAGE
