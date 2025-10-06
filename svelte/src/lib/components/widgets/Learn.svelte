<script lang="ts">
  import { Headline, Title2 } from "$lib/components/typography";
  import { HStack, VStack } from "../UI";
  import CardClickable from "../cards/CardClickable.svelte";

  const { cardsCount = 0 } = $props();
</script>

<HStack>
  <Title2>Слова</Title2>

  <CardClickable href="flashcards/learn">
    <Headline>Карточек посмотреть</Headline>
    <VStack>
      <Title2>
        {cardsCount}
      </Title2>
    </VStack>
  </CardClickable>
</HStack>
