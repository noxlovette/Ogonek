<script lang="ts">
  import { enhance } from "$app/forms";
  import { goto } from "$app/navigation";
  import {
    Divider,
    LargeTitle,
    Lessons,
    Merger,
    Sidebar,
    Toolbar,
    UniButton,
  } from "$lib/components";
  import {
    assigneeStore,
    clearUser,
    currentPage,
    notification,
    pageSize,
    searchTerm,
  } from "$lib/stores";
  import { enhanceForm } from "$lib/utils";
  import { LogOut } from "@lucide/svelte";

  const { children } = $props();
</script>

<Toolbar>
  <LargeTitle>Настройки</LargeTitle>
  <Divider />
  <form
    action="?/logout"
    method="POST"
    class="flex flex-col"
    use:enhance={enhanceForm({
      handlers: {
        redirect: async (result) => {
          clearUser();
          assigneeStore.set("");
          pageSize.set(20);
          currentPage.reset();
          searchTerm.set("");
          notification.set({ message: "Bye!", type: "success" });
          goto(result.location);
        },
      },
    })}
  >
    <Merger>
      <UniButton variant="danger" type="submit" Icon={LogOut} content="Уйти"
      ></UniButton>
    </Merger>
  </form></Toolbar
>
{@render children?.()}
