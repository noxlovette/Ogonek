<script lang="ts">
  import {
    UniButton,
    VStack,
    HStack,
    Input,
    Editor,
    Toolbar,
    LargeTitle,
    Divider,
    Merger,
    SaveButton,
    CancelButton,
    DeleteButton,
  } from "$lib/components";
  import { MegaphoneOff, Megaphone } from "lucide-svelte";
  import { enhance } from "$app/forms";
  import type { PageProps } from "./$types";
  import { enhanceForm } from "$lib/utils";

  let { data }: PageProps = $props();

  let { metaDescription } = data.content;

  let title: string = $state(data.content.title);

  let markdown = $state(data.content.markdown);
  function generateSlug(title: string) {
    return title
      .toLowerCase()
      .replace(/[^a-z0-9\s-]/g, "")
      .replace(/\s+/g, "-")
      .replace(/-+/g, "-")
      .trim();
  }

  const slug = $derived(generateSlug(title));
</script>

<form
  method="POST"
  action="?/update"
  use:enhance={enhanceForm({
    messages: {
      redirect: "Updated",
      success: data.content.status == "draft" ? "Published" : "Unpublished",
    },
    shouldUpdate: true,
  })}
>
  <Toolbar>
    <HStack>
      <VStack>
        <LargeTitle>Edit Content</LargeTitle>
        <Divider />
        <VStack>
          <Merger>
            {#if data.content.status == "published"}
              <UniButton
                type="submit"
                content="Unpublish"
                formaction="?/unpublish"
                Icon={MegaphoneOff}
              ></UniButton>
            {:else}
              <UniButton
                type="submit"
                formaction="?/publish"
                Icon={Megaphone}
                content="Publish"
              ></UniButton>
            {/if}
          </Merger>
          <Merger>
            <DeleteButton />
            <CancelButton />
            <SaveButton />
          </Merger>
        </VStack>
      </VStack>
    </HStack>
  </Toolbar>

  <HStack>
    <Input
      name="title"
      bind:value={title}
      placeholder="Enter content title"
      required
    />
    <Input
      name="metaDescription"
      value={metaDescription}
      placeholder="Brief description for SEO"
    />
    <input type="hidden" name="slug" value={slug} />
    <Editor bind:markdownContent={markdown} />
    <input type="hidden" name="markdown" value={markdown} />
  </HStack>
</form>
