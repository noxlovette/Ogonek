<script lang="ts">
  import { m } from "$lib/paraglide/messages";
  import { ArrowBigRight, Clock, Target, Zap, Calendar } from "lucide-svelte";
  import UniButton from "../UI/UniButton.svelte";
  import { Title2 } from "$lib/components/typography";

  const {
    cardsCount = 0,
    streakDays = 0,
    todayStudied = 0,
    dailyGoal = 10,
  } = $props();

  const progressPercentage = $derived(
    Math.min((todayStudied / dailyGoal) * 100, 100),
  );
</script>

<div class="flex flex-col gap-4">
  <Title2>{m.flashcards()}</Title2>

  <!-- Main Review Card -->
  <div
    class="bg-default ring-default relative overflow-hidden rounded-xl p-6 shadow-sm"
  >
    <div class="mb-4 flex items-center justify-between">
      <div>
        <div class="mb-2 flex items-center gap-2">
          <Clock class="h-5 w-5 text-stone-600" />
          <span class="text-sm font-medium text-stone-600"
            >{m.sound_direct_tern_startle()}</span
          >
        </div>
        <div class="flex items-baseline gap-2">
          <span class="text-3xl font-bold">{cardsCount}</span>
          <span class="text-lg text-stone-600"
            >{m.some_happy_cat({ count: cardsCount })}</span
          >
        </div>
      </div>
    </div>

    {#if cardsCount > 0}
      <UniButton variant="primary" Icon={ArrowBigRight} href="flashcards/learn">
        {m.helpful_slow_flea_catch()}
      </UniButton>
    {:else}
      <div class="py-4 text-center">
        <Zap class="mx-auto mb-2 h-8 w-8 text-green-500" />
        <p class="text-stone-600">{m.livid_trite_squirrel_cuddle()}</p>
      </div>
    {/if}
  </div>

  <!-- Stats Grid -->
  <div class="grid grid-cols-2 gap-4">
    <!-- Daily Progress -->
    <div class="bg-default ring-default rounded-lg p-4">
      <div class="mb-3 flex items-center gap-2">
        <Target class="h-4 w-4 text-amber-600" />
        <span class="text-sm font-medium text-stone-600">{m.today()}</span>
      </div>

      <div class="space-y-2">
        <div class="flex justify-between text-sm">
          <span class="">{todayStudied} / {dailyGoal}</span>
          <span class="text-stone-600">{Math.round(progressPercentage)}%</span>
        </div>

        <!-- Progress bar -->
        <div class="h-2 w-full rounded-full bg-stone-200 dark:bg-stone-800">
          <div
            class="h-2 rounded-full bg-gradient-to-r from-amber-500 to-amber-600 duration-300"
            style="width: {progressPercentage}%"
          ></div>
        </div>
      </div>
    </div>

    <!-- Streak Counter -->
    <div class="ring-default bg-default rounded-lg p-4">
      <div class="mb-3 flex items-center gap-2">
        <Calendar class="h-4 w-4 text-orange-600" />
        <span class="text-sm font-medium text-stone-600"
          >{m.just_loose_canary_tear()}</span
        >
      </div>

      <div class="flex items-baseline gap-1">
        <span class="text-2xl font-bold">{streakDays}</span>
        <span class="text-sm text-stone-600"
          >{m.days({ count: streakDays })}</span
        >
      </div>
    </div>
  </div>
</div>
