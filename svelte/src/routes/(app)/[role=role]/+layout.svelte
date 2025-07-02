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
    Rightbar,
    RecentLessons,
    QuickAdd,
    MobileMenu,
    StudentFilter,
  } from "$lib/components";
  import {
    lessonStore,
    studentStore,
    setUser,
    setProfile,
    setTeacherData,
    sidebar,
    mobileMenuOpen,
  } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Word, Student } from "$lib/types";
  import ThemeToggler from "$lib/components/UI/interactive/ThemeToggler.svelte";
  import Clock from "$lib/components/UI/Clock.svelte";
  import { Menu } from "lucide-svelte";

  let { data, children } = $props();
  const role = page.params.role;

  let elementsLeft = $state([Dashboard, Todo, Lessons, Words, Zoom, Settings]);
  let elementsRight = $state([UsefulLinks, WordOfTheDay, RecentLessons]);

  if (role === "t") {
    elementsLeft = [Dashboard, Todo, Lessons, Students, Words, Settings];
    elementsRight = [QuickAdd, StudentFilter];
  }

  lessonStore.setLessons(data.lessons.data);
  studentStore.setStudents(data.students);
  setContext<Promise<Word>>("word", data.word);
  setContext<Student[]>("students", data.students);
  setContext<number>("lessonCount", data.lessons.count);
  setContext<number>("deckCount", data.deckCount);
  setContext<number>("taskCount", data.tasks.count);

  setUser(data.user);
  setProfile(data.profile);
  setTeacherData(data.teacherData);
</script>

<div class="flex flex-row">
  <div class=" hidden flex-col md:block {$sidebar ? 'w-max' : 'w-1/6'}">
    <Sidebar elements={elementsLeft} />
  </div>
  <WorkArea>
    {@render children?.()}
  </WorkArea>
  <div class="hidden w-1/6 pl-4 md:block">
    <div class="flex items-baseline justify-between pb-5">
      <Clock></Clock>
      <ThemeToggler />
    </div>
    <Rightbar elements={elementsRight}></Rightbar>
  </div>
  <button
    aria-label={$mobileMenuOpen ? "Close menu" : "Open menu"}
    class="absolute right-0 z-50 flex flex-col items-center justify-center rounded-md p-2 transition-colors hover:bg-stone-200 md:hidden dark:hover:bg-stone-700"
    onclick={() => mobileMenuOpen.toggle()}
  >
    <Menu />
  </button>
  <MobileMenu elements={elementsLeft} />
</div>
<svelte:head>
  <title>Tasks</title>
</svelte:head>
