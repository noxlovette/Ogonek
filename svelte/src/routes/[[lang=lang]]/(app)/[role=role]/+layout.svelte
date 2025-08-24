<script lang="ts">
  import {
    Dashboard,
    Todo,
    Lessons,
    Words,
    Settings,
    Students,
    Zoom,
    Sidebar,
    WorkArea,
    UsefulLinks,
    WordOfTheDay,
    QuickAdd,
    MobileMenu,
    StudentFilter,
  } from "$lib/components";
  import {
    studentStore,
    setUser,
    setProfile,
    sidebar,
    mobileMenuOpen,
  } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Word, Student } from "$lib/types";
  import { Menu } from "lucide-svelte";
  import LoadingOverlay from "$lib/components/UI/LoadingOverlay.svelte";

  let { data, children } = $props();
  const role = page.params.role;

  let elementsLeft = $state([Dashboard, Todo, Lessons, Words, Zoom, Settings]);
  let elementsRight = $state([UsefulLinks, WordOfTheDay]);

  if (role === "t") {
    elementsLeft = [Dashboard, Todo, Lessons, Students, Words, Settings];
    elementsRight = [QuickAdd, StudentFilter];
  }

  studentStore.setStudents(data.students);
  setContext<Promise<Word>>("word", data.word);
  setContext<string | null>("callURL", data.callURL);
  setContext<Student[]>("students", data.students);
  setContext<number>("lessonCount", data.badges.unseenLessons);
  setContext<number>("deckCount", data.badges.unseenDecks);
  setContext<number>("taskCount", data.badges.unseenTasks);

  setUser(data.user);
  setProfile(data.profile);
</script>

<div class="flex flex-row">
  <div class="hidden w-max flex-col md:block">
    <Sidebar elements={elementsLeft} />
  </div>
  <WorkArea>
    {@render children?.()}
  </WorkArea>
  <button
    aria-label={$mobileMenuOpen ? "Close menu" : "Open menu"}
    class="absolute right-0 z-50 flex flex-col items-center justify-center rounded-md p-2 hover:bg-stone-200 md:hidden dark:hover:bg-stone-700"
    onclick={() => mobileMenuOpen.toggle()}
  >
    <Menu />
  </button>
  <MobileMenu elements={elementsLeft} />
  <LoadingOverlay />
</div>

<svelte:head>
  <title>Ogonek | Main</title>
</svelte:head>
