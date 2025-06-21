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
  } from "$lib/stores";

  import { page } from "$app/state";
  import { setContext } from "svelte";
  import type { Word, Student } from "$lib/types";
  import ThemeToggler from "$lib/components/UI/interactive/ThemeToggler.svelte";
  import Clock from "$lib/components/UI/Clock.svelte";

  let { data, children } = $props();
  const role = page.params.role;

  let elementsLeft = $state([Dashboard, Todo, Lessons, Words, Zoom, Settings]);
  let elementsRight = $state([UsefulLinks, WordOfTheDay, RecentLessons]);

  if (role === "t") {
    elementsLeft = [Dashboard, Todo, Lessons, Students, Words, Settings];
    elementsRight = [QuickAdd, StudentFilter];
  }

  lessonStore.setLessons(data.lessons);
  studentStore.setStudents(data.students);
  setContext<Promise<Word>>("word", data.word);
  setContext<Student[]>("students", data.students);

  setUser(data.user);
  setProfile(data.profile);
  setTeacherData(data.teacherData);
</script>

<div class="flex flex-row py-2">
  <div class="flex w-1/6 flex-col">
    <a href="/" class="font-serif text-2xl font-bold">Ogonek</a>
    <Sidebar elements={elementsLeft} />
  </div>
  <WorkArea>
    {@render children?.()}
  </WorkArea>
  <div class="w-1/6 pl-4">
    <div class="flex items-baseline justify-between pb-5">
      <Clock></Clock>
      <ThemeToggler />
    </div>
    <Rightbar elements={elementsRight}></Rightbar>
  </div>
  <MobileMenu elements={elementsLeft} />
</div>
<svelte:head>
  <title>Tasks</title>
</svelte:head>
