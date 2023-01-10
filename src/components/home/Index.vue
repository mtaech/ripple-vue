<script setup lang="ts">
import {MenuOption, NIcon, NLayout, NLayoutFooter, NLayoutHeader} from "naive-ui";
import {Component} from "vue";
import {BookOutline as BookIcon, CreateOutline} from '@vicons/ionicons5'
import {RouterLink} from "vue-router";

const activeKey = ref<string>("");
function renderIcon (icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions: MenuOption[] = [
  {
    label: () =>
        h(
            RouterLink,
            {
              to: {
                name: 'home'
              }
            },
            { default: () => '小说管理' }
        ),
    key: 'dance-dance-dance',
  },{
    label: '设定管理',
    key: 'dance-dance-dance',
  }
]
</script>
<template>
  <n-layout position="absolute">
    <n-layout-header class="default-content"
                     style="height: 64px; padding: 24px 24px 24px 0;"
                     :bordered="false">
      <n-menu  mode="horizontal" :options="menuOptions"/>
    </n-layout-header>
    <n-layout has-sider position="absolute" style="top: 64px; bottom: 64px">
      <n-layout content-style="padding: 24px;">
       <router-view/>
      </n-layout>
    </n-layout>
    <n-layout-footer
        bordered
        position="absolute"
        style="height: 64px; padding: 24px"
    >
    </n-layout-footer>
  </n-layout>
</template>

<script lang="ts">
export default {
  name: "Home",

}
</script>
<style scoped>

.layout-demo {
  height: 500px;
  background: var(--color-fill-2);
  border: 1px solid var(--color-border);
}

.layout-demo :deep(.arco-layout-sider) .logo {
  height: 32px;
  margin: 12px 8px;
  background: rgba(255, 255, 255, 0.2);
}

.layout-demo :deep(.arco-layout-sider-light) .logo {
  background: var(--color-fill-2);
}

.layout-demo :deep(.arco-layout-header) {
  height: 64px;
  line-height: 64px;
  background: var(--color-bg-3);
}

.layout-demo :deep(.arco-layout-footer) {
  height: 48px;
  color: var(--color-text-2);
  font-weight: 400;
  font-size: 14px;
  line-height: 48px;
}

.layout-demo :deep(.arco-layout-content) {
  color: var(--color-text-2);
  font-weight: 400;
  font-size: 14px;
  background: var(--color-bg-3);
}

.layout-demo :deep(.arco-layout-footer),
.layout-demo :deep(.arco-layout-content) {
  display: flex;
  flex-direction: column;
  justify-content: center;
  color: var(--color-white);
  font-size: 16px;
  font-stretch: condensed;
  text-align: center;
}

</style>