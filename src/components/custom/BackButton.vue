<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { RouterLink, useRoute } from "vue-router";

const route = useRoute();

const getParentPath = () => {
  const currentPath = route.path;

  const normalizedPath = currentPath.endsWith("/")
    ? currentPath
    : currentPath + "/";

  if (normalizedPath.includes("/results/")) {
    return normalizedPath
      .substring(0, normalizedPath.lastIndexOf("/results/"))
      .replace(/\/$/, "");
  } else if (normalizedPath.startsWith("/competition/")) {
    return "/";
  } else if (normalizedPath === "/custom/" || normalizedPath === "/custom") {
    return "/";
  }

  return "/";
};
</script>

<template>
  <RouterLink :to="getParentPath()" custom v-slot="{ navigate }">
    <Button variant="outline" class="absolute m-2" @click="navigate">
      Back
    </Button>
  </RouterLink>
</template>
