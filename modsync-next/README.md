# modsync-next

## 开发与发布

`pnpm dev:desktop` 用于开发：它会启动 Vite，并由桌面程序加载
`http://localhost:1420`。

不要直接运行 `cargo build` 生成的调试二进制文件；该文件依赖上述开发服务器。
发布版本请在仓库根目录执行：

```powershell
pnpm build:desktop
```

该命令会先构建前端并将其嵌入 Tauri 应用。Windows 安装包位于
`target\\release\\bundle\\`，不需要任何 localhost 服务。
