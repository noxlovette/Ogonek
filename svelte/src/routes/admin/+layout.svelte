<script lang="ts">
  import { page } from "$app/stores";
  import {
    Body,
    Content,
    Dashboard,
    HStack,
    Sidebar,
    VStack,
    WorkArea,
  } from "$lib/components";
  import { Settings, FileText, Home } from "lucide-svelte";
  import type { LayoutProps } from "./$types";
  import Loader from "$lib/components/UI/navigation/Loader.svelte";

  let { data, children }: LayoutProps = $props();

  const navItems = [
    { href: "/admin", label: "Dashboard", icon: Home },
    { href: "/admin/content", label: "Content", icon: FileText },
  ];

  function isActiveRoute(href: string) {
    if (href === "/admin") {
      return $page.url.pathname === "/admin";
    }
    return $page.url.pathname.startsWith(href);
  }
</script>

<div class="md:hidden">
  <Body>Not available on mobile</Body>
</div>
<div class="hidden flex-row gap-4 p-2 md:flex md:gap-6 md:p-5 lg:gap-8 lg:p-6">
  <div class="hidden w-max flex-col md:block">
    <Sidebar><Dashboard /><Content /></Sidebar>
  </div>
  <WorkArea>
    {@render children?.()}
  </WorkArea>

  <Loader />
</div>

<svelte:head>
  <title>Ogonek | Admin</title>
</svelte:head>
