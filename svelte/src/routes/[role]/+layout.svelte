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

<Sidebar elements={elementsLeft} />
<WorkArea>
  {@render children?.()}
</WorkArea>

<Rightbar elements={elementsRight}></Rightbar>
<MobileMenu elements={elementsLeft} />

<svelte:head>
  <title>Tasks</title>
</svelte:head>
