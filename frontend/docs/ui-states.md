# 全局加载与异常状态组件（#41）

状态组件位于 `src/components/common/`，使用项目的主题变量、Syne 标题、Inter 正文和 Base UI 按钮。默认文案与当前项目一致为英文，可通过 props 自定义中文或其他文案。

## 接入方式

```tsx
import { Loading } from '@/components/common/Loading'
import { EmptyState } from '@/components/common/EmptyState'
import { ErrorState } from '@/components/common/ErrorState'

if (isLoading) return <Loading message="正在查找菜谱…" />

if (error) {
  return (
    <ErrorState
      title="加载失败"
      message={error.message}
      onRetry={refetch}
      isRetrying={isRefetching}
      retryLabel="重试"
      retryingLabel="正在重试…"
    />
  )
}

if (recipes.length === 0) {
  return (
    <EmptyState
      title="未找到相关菜谱"
      message="试试其他食材，或调整筛选条件。"
    />
  )
}
```

- `Loading`：可自定义 `message` 和 `className`；包含转圈动画、读屏状态提示，并尊重系统减少动画的设置。
- `EmptyState`：可自定义 `title`、`message`、`icon`、`action` 和 `className`。`action` 可传入现有 `Button` 或路由链接。
- `ErrorState`：接收 `message` 和必填的 `onRetry` 回调；可自定义 `title`、`retryLabel`、`retryingLabel`、`className`。请求状态由调用方管理；设置 `isRetrying` 会禁用重试按钮并显示加载反馈。请传入适合用户阅读的报错文案，避免直接显示堆栈。
- 组件不发起网络请求，可复用于菜谱列表、详情和其他页面。

## 路由与本地验证

- `src/pages/NotFound.tsx` 已接入 `App.tsx` 的 `*` 路由；所有未匹配路径显示 404，可返回首页。
- 启动 `npm run dev` 后访问 `/dev/ui-states`，可查看加载、空状态、错误状态，切换浅色／深色主题，并验证重试中的禁用反馈、重试成功和重置流程。
- 预览页中的重试为本地模拟，900ms 后切换到完成状态；离开页面时会清理计时器。
- 预览路由仅在开发模式启用，生产环境下访问该地址也会显示 404。
- 可在浏览器分别检查手机宽度、减少动画设置，并访问任意未知路径验证 404 返回首页。
- 完成修改后运行 `npm run build` 检查类型与生产构建，运行 `npm run lint` 检查代码规范。
