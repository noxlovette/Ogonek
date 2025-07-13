<script lang="ts">
  import { m } from "$lib/paraglide/messages";
  import {
    ArrowBigRight,
    Clock,
    Target,
    Zap,
    Calendar,
    PlusCircle,
    WholeWord,
  } from "lucide-svelte";
  import UniButton from "../UniButton.svelte";
  import { H2 } from "$lib/components/typography";

  const {
    cardsCount = 0,
    streakDays = 0,
    todayStudied = 0,
    dailyGoal = 20,
  } = $props();

  // Calculate urgency level for visual feedback
  const urgencyLevel = $derived(
    cardsCount > 50 ? "high" : cardsCount > 20 ? "medium" : "low",
  );
  const progressPercentage = $derived(
    Math.min((todayStudied / dailyGoal) * 100, 100),
  );
</script>

<div class="flex flex-col gap-6">
  <H2>Learn</H2>

  <!-- Main Review Card -->
  <div
    class="relative overflow-hidden rounded-xl border border-stone-200 bg-gradient-to-br from-stone-50 to-stone-100 p-6 shadow-sm"
  >
    <div class="mb-4 flex items-center justify-between">
      <div>
        <div class="mb-2 flex items-center gap-2">
          <Clock class="h-5 w-5 text-stone-600" />
          <span class="text-sm font-medium text-stone-600">Due for Review</span>
        </div>
        <div class="flex items-baseline gap-2">
          <span class="text-3xl font-bold text-stone-900">{cardsCount}</span>
          <span class="text-lg text-stone-600">cards</span>
        </div>
      </div>

      <!-- Urgency indicator -->
      <div class="flex flex-col items-end">
        <div
          class="mb-2 h-3 w-3 rounded-full {urgencyLevel === 'high'
            ? 'bg-red-500'
            : urgencyLevel === 'medium'
              ? 'bg-amber-500'
              : 'bg-green-500'}"
        ></div>
        <span class="text-xs text-stone-500 capitalize"
          >{urgencyLevel} priority</span
        >
      </div>
    </div>

    {#if cardsCount > 0}
      <UniButton variant="primary" Icon={ArrowBigRight} href="words/learn">
        {m.helpful_slow_flea_catch()}
      </UniButton>
    {:else}
      <div class="py-4 text-center">
        <Zap class="mx-auto mb-2 h-8 w-8 text-green-500" />
        <p class="text-stone-600">All caught up! 🎉</p>
      </div>
    {/if}
  </div>

  <!-- Stats Grid -->
  <div class="grid grid-cols-2 gap-4">
    <!-- Daily Progress -->
    <div class="rounded-lg border border-stone-200 bg-white p-4">
      <div class="mb-3 flex items-center gap-2">
        <Target class="h-4 w-4 text-amber-600" />
        <span class="text-sm font-medium text-stone-700">Today's Progress</span>
      </div>

      <div class="space-y-2">
        <div class="flex justify-between text-sm">
          <span class="text-stone-600">{todayStudied} / {dailyGoal}</span>
          <span class="text-stone-600">{Math.round(progressPercentage)}%</span>
        </div>

        <!-- Progress bar -->
        <div class="h-2 w-full rounded-full bg-stone-200">
          <div
            class="h-2 rounded-full bg-gradient-to-r from-amber-500 to-amber-600 transition-all duration-300"
            style="width: {progressPercentage}%"
          ></div>
        </div>
      </div>
    </div>

    <!-- Streak Counter -->
    <div class="rounded-lg border border-stone-200 bg-white p-4">
      <div class="mb-3 flex items-center gap-2">
        <Calendar class="h-4 w-4 text-orange-600" />
        <span class="text-sm font-medium text-stone-700">Streak</span>
      </div>

      <div class="flex items-baseline gap-1">
        <span class="text-2xl font-bold text-stone-900">{streakDays}</span>
        <span class="text-sm text-stone-600">days</span>
      </div>

      {#if streakDays > 0}
        <div class="mt-2 flex gap-1">
          {#each Array(Math.min(streakDays, 7)) as _, i}
            <div class="h-2 w-2 rounded-full bg-orange-500"></div>
          {/each}
          {#if streakDays > 7}
            <span class="ml-1 text-xs text-stone-500">+{streakDays - 7}</span>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- Quick Actions (if you want to add more later) -->
  {#if cardsCount === 0}
    <div class="rounded-lg border border-stone-200 bg-stone-50 p-4">
      <h3 class="mb-2 font-medium text-stone-800">What's next?</h3>
      <div class="flex gap-2">
        <UniButton Icon={WholeWord} variant="secondary" href="/words/browse">
          Browse Deck
        </UniButton>
        <UniButton variant="secondary" Icon={PlusCircle} href="/words/add"
          >Add Cards</UniButton
        >
      </div>
    </div>
  {/if}
</div>
