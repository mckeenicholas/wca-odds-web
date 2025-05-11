<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { getParentPath } from "@/lib/utils";
import { RouterLink, useRoute } from "vue-router";

const route = useRoute();
const parentPath = getParentPath(route.path);
</script>

<template>
  <RouterLink :to="parentPath" custom v-slot="{ navigate }">
    <Button variant="outline" class="absolute m-2" @click="navigate">
      Back
    </Button>
  </RouterLink>
</template>
