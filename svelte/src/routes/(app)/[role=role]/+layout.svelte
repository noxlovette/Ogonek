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
    Account,
    Notifications,
    Teacher,
  } from "$lib/components";
  import { studentStore, setUser, setProfile } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Student } from "$lib/types";
  import Loader from "$lib/components/UI/navigation/Loader.svelte";

  const isSettings = $derived(page.url.pathname.includes("settings"));
  let { data, children } = $props();
  const role = page.params.role;

  const { students } = $derived(data);

  let elementsMobile = $derived(
    isSettings
      ? [Dashboard, Account, Notifications, Teacher]
      : [Dashboard, Todo, Lessons, Words],
  );

  studentStore.setStudents(data.students);
  setContext<Student[]>("students", data.students);
  setContext<number>("lessonCount", data.badges.unseenLessons);
  setContext<number>("deckCount", data.badges.unseenDecks);
  setContext<number>("taskCount", data.badges.unseenTasks);

  setUser(data.user);
  setProfile(data.profile);
</script>

<div class="flex flex-row gap-4 p-2 md:gap-6 md:p-5 lg:gap-8 lg:p-6">
  <div class="gap-default hidden flex-col md:flex">
    <Sidebar
      ><Dashboard />
      <HLine />
      {#if !isSettings}
        <Todo />
        <Lessons />
        <Words />
        <HLine></HLine>
        {#if role == "s"}
          <Zoom href={data.callURL ?? "https://zoom.us"} />
        {:else}
          <Students />
        {/if}
        {#if role != "s"}
          <Calendar />
        {/if}
      {:else}
        <Account />
        <Notifications />
        <Teacher />
      {/if}
    </Sidebar>
    {#if !isSettings}
      <Sidebar>
        {#if role == "s"}
          <UsefulLinks />
        {:else}
          <StudentFilter {students} />
          <QuickAdd />
          <Divider></Divider>
        {/if}
      </Sidebar>
    {/if}
  </div>
  <WorkArea>
    {@render children?.()}
  </WorkArea>

  <Loader />
</div>
<MobileMenu elements={elementsMobile} />

<svelte:head>
  <title>Ogonek</title>
</svelte:head>
