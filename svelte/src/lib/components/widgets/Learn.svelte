<script lang="ts">
  import { m } from "$lib/paraglide/messages";
  import { Headline, Subheadline, Title2 } from "$lib/components/typography";
  import { HStack, VStack } from "../UI";
  import CardClickable from "../cards/CardClickable.svelte";

  const { cardsCount = 0 } = $props();
</script>

<HStack>
  <Title2>{m.flashcards()}</Title2>

  <CardClickable href="flashcards/learn">
    <Headline>
      {m.sound_direct_tern_startle()}
    </Headline>
    <VStack>
      <Title2>
        {cardsCount}
      </Title2>
      <Subheadline>
        {m.some_happy_cat({ count: cardsCount })}
      </Subheadline>
    </VStack>
  </CardClickable>
</HStack>
