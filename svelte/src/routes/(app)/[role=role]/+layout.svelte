<script lang="ts">
  import {
    Dashboard,
    Todo,
    Lessons,
    Words,
    Students,
    Zoom,
    Sidebar,
    WorkArea,
    UsefulLinks,
    QuickAdd,
    MobileMenu,
    StudentFilter,
    Calendar,
  } from "$lib/components";
  import { studentStore, setUser, setProfile } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Student } from "$lib/types/index.js";
  import Divider from "$lib/components/UI/toolbar/Divider.svelte";
  import Loader from "$lib/components/UI/navigation/Loader.svelte";
  import { env } from "$env/dynamic/public";

  let { data, children } = $props();
  const role = page.params.role;

  let elementsMobile = [Dashboard, Todo, Lessons, Words];

  studentStore.setStudents(data.students);
  setContext<string | null>("callURL", data.callURL ?? "https://zoom.us");
  setContext<Student[]>("students", data.students);
  setContext<number>("lessonCount", data.badges.unseenLessons);
  setContext<number>("deckCount", data.badges.unseenDecks);
  setContext<number>("taskCount", data.badges.unseenTasks);

  setUser(data.user);
  setProfile(data.profile);
</script>

<div class="flex flex-row gap-4 p-2 md:gap-6 md:p-5 lg:gap-8 lg:p-6">
  <div class="hidden w-max flex-col md:block">
    <Sidebar
      ><Dashboard />
      <Todo />
      <Lessons />
      <Words />
      {#if role == "s"}
        <Zoom />
      {:else}
        <Students />
      {/if}
      {#if !env.PUBLIC_TURN_OFF_CALENDAR}
        <Calendar />
      {/if}

      <Divider />
      {#if role == "s"}
        <UsefulLinks />
      {:else}
        <StudentFilter />
        <QuickAdd />
        <Divider></Divider>
      {/if}
    </Sidebar>
  </div>
  <WorkArea>
    {@render children?.()}
  </WorkArea>

  <Loader />
</div>
<MobileMenu elements={elementsMobile} />

<svelte:head>
  <title>Ogonek | Main</title>
</svelte:head>
