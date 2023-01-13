<template>
  <a-layout class="layout">
    <a-layout-header>
      <div class="header-menu">
        <a-menu mode="horizontal" :default-selected-keys="['1']">
          <a-menu-item key="1">
            <template #icon>
              <icon-home/>
            </template>
            首页
          </a-menu-item>
          <a-menu-item key="2">Solution</a-menu-item>
        </a-menu>
      </div>
    </a-layout-header>
    <a-layout-content>
      <a-divider style="margin:15px 0;"/>
      <router-view/>
    </a-layout-content>
    <a-layout-footer>
      Footer
    </a-layout-footer>
  </a-layout>
</template>

<script lang="ts">
export default {
  name: "Home",

}
</script>
<style scoped>
.layout {
  width: 100vw;
  height: 100vh;
  background: var(--color-fill-2);
}

.layout :deep(.arco-layout-sider) .logo {
  height: 32px;
  margin: 12px 8px;
  background: rgba(255, 255, 255, 0.2);
}

.layout :deep(.arco-layout-sider-light) .logo {
  background: var(--color-fill-2);
}

.layout :deep(.arco-layout-header) {
  height: 54px;
  line-height: 54px;
  background: var(--color-bg-3);
}
.header-menu{
  width: 60vw;
  margin: auto;
}

.layout :deep(.arco-layout-footer) {
  height: 48px;
  color: var(--color-text-2);
  background: var(--color-bg-3);
  font-weight: 400;
  font-size: 14px;
  line-height: 48px;
}

.layout :deep(.arco-layout-content) {
  color: var(--color-text-2);
  font-weight: 400;
  font-size: 16px;
  background: var(--color-bg-3);
}

.layout :deep(.arco-layout-footer),
.layout :deep(.arco-layout-content) {
  display: flex;
  flex-direction: column;
  color: var(--color-white);
  font-size: 16px;
  font-stretch: condensed;
}
.layout :deep(.arco-menu-icon){
  margin-right: 5px !important;
}

</style>