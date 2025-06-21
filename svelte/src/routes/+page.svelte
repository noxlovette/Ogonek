<script lang="ts">
  import { Header, IntroCard, UniButton } from "$lib/components";
  import { Flame, FlameKindling } from "lucide-svelte";
</script>

<Header />
<div class="flex w-full flex-col items-center space-y-4 md:p-6">
  <h1 class="text-3xl font-bold tracking-tighter md:text-4xl lg:text-6xl">
    Welcome to <span class="font-serif italic">Ogonek</span>
  </h1>
  <grid class="grid gap-3 md:grid-cols-3">
    <IntroCard title="Lessons"
      >Enjoy the freedom of markdown to keep track of each lesson you have, get
      control of your money and time investment.</IntroCard
    >
    <IntroCard title="Homework"
      >Upload and download files, set deadlines, create and receive detailed
      tasks to get going in your studies.</IntroCard
    >
    <IntroCard title="Flashcards"
      >Imagine if Quizlet and Anki had a child – that is what is waiting for you
      in Ogonek's spaced-repetition study logic.</IntroCard
    >
  </grid>
  <div class="grid grid-cols-2 gap-3">
    <UniButton
      Icon={Flame}
      iconPosition="right"
      href="/auth/login"
      variant="secondary">Log In</UniButton
    >
    <UniButton Icon={FlameKindling} href="/auth/signup" variant="primary">
      Sign up</UniButton
    >
  </div>
</div>
