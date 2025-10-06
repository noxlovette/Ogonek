<script lang="ts">
  import { Caption1, LargeTitle } from "$lib/components";

  import { page } from "$app/state";

  const isLogin = $derived(
    page.url.href.includes("login") || page.url.href.includes("bind"),
  );

  let { children } = $props();
</script>

<div class="gap-default mx-auto flex flex-col items-center justify-center">
  <LargeTitle styling="text-center"
    >{isLogin ? "Мы вас ждали" : "Добро пожаловать"}</LargeTitle
  >
  <Caption1>
    {isLogin ? "Нет аккаунта?" : "Есть аккаунт?"}
    <a
      data-cy={isLogin ? "signup-button" : "login-button"}
      href={isLogin ? "/signup" : "/login"}
      class="text-accent">{isLogin ? "Давайте делать" : "Вам сюда"}</a
    >
  </Caption1>

  {@render children?.()}
</div>
