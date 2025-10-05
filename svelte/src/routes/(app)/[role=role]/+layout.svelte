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
    Divider,
    HLine,
    Caption1,
  } from "$lib/components";
  import { studentStore, setUser, setProfile } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Student } from "$lib/types";
  import Loader from "$lib/components/UI/navigation/Loader.svelte";

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
  <div class="gap-default hidden w-max flex-col md:flex">
    <Sidebar
      ><Dashboard />
      <HLine />
      <Todo />
      <Lessons />
      <Words />
      <HLine></HLine>
      {#if role == "s"}
        <Zoom />
      {:else}
        <Students />
      {/if}
      {#if role != "s"}
        <Calendar />
      {/if}
    </Sidebar>
    <Sidebar>
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
